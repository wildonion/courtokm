

use std::{collections::HashMap};
use futures::future::{BoxFuture, FutureExt};
use tokio::net::tcp;
use serde::{Serialize, Deserialize};

// static requires constant value and constant values must be only stack data like &[] and &str otherwise
// we're not allowed to have heap data types like Vec, String, Box, Arc, Mutex in const and static as value
// also in order to mutate an static we should wrap around the arc and mutex to do so but inside lazy
// str is ?Sized and it must be behind pointer cause it's an slice of String, same for [] 
// generally only &[] and &str are allowed to be in const and static value, using other types than these
// requires a global static lazy arc mutex like so:
type SafeMap = Lazy<std::sync::Arc<tokio::sync::Mutex<HashMap<String, String>>>>;
pub static GLOBALMAP: SafeMap = Lazy::new(||[
    std::sync::Arc::new(
        tokio::sync::Mutex::new(
            HashMap::new()
        )
    )
]);
pub static ONTHEHEAP: &[&str] = CONSTVALUE;
pub const CONSTVALUE: &[&str] = &["wildonion"];
pub const CHARSET: &[u8] = b"0123456789";


// -----------------------------------
// handling a recursive async function
// -----------------------------------
// https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html
// NOTE - Future trait is an object safe trait thus we have to Box it with dyn keyword to have kinda a pointer to the heap where the object is allocated in runtime
// NOTE - a recursive `async fn` will always return a Future object which must be rewritten to return a boxed `dyn Future` to prevent infinite size allocation in runtime from heppaneing some kinda maximum recursion depth exceeded prevention process
// the return type can also be ... -> impl std::future::Future<Output=usize>
// which implements the future trait for the usize output also BoxFuture<'static, usize>
// is a pinned Box under the hood because in order to return a future as a type
// we have to return its pinned pointer since future objects are traits and 
// traits are not sized at compile time thus we have to put them inside the 
// Box or use &dyn to return them as a type and for the future traits we have
// to pin them into the ram in order to be able to solve them later so we must 
// return the pinned Box (Box in here is a smart pointer points to the future)
// or use impl Trait in function return signature. 
//
// async block needs to be pinned into the ram and since they are traits of 
// the Future their pointer will be either Box<dyn Trait> or &dyn Trait, 
// to pin them into the ram to solve them later.
//
// since async blocks are of type Future trait in roder to return them
// as a type their pointer either Box<dyn Trait> or &dyn Trait must be
// pinned into the ram to let us solve them later because rust doesn't 
// have gc and it'll drop the type after it moved into the new scope or
// another type thus for the future objects we must pin them to ram and 
// tell rust hey we're moving this in other scopes but don't drop it because
// we pinned it to the ram to solve it in other scopes, also it must have
// valid lifetime during the the entire lifetime of the app.
//
// BoxFuture<'fut, ()> is Pin<alloc::boxed::Box<dyn Future<Output=()> + Send + Sync + 'fut>>
pub fn async_gen_random_idx(idx: usize) -> BoxFuture<'static, usize>{ // NOTE - pub type BoxFuture<'a, T> = Pin<alloc::boxed::Box<dyn Future<Output = T> + Send + 'a>>
    async move{
        if idx <= CHARSET.len(){
            idx
        } else{
            gen_random_idx(rand::random::<u8>() as usize)
        }
    }.boxed() // wrap the future in a Box, pinning it
}
pub fn ret_boxed_future() -> std::pin::Pin<Box<dyn std::future::Future<Output=()>>>{ // Pin takes a pointer to the type and since traits are dynamic types thir pointer can be either &dyn ... or Box<dyn...>
    // ret future as a pinned box means pinning the pointer of future trait into the ram so they can't move
    Box::pin(async move{ // pinning the box pointer of async block into the ram to solve it later 
        ()
    })
}

// recursive random index generator
pub fn gen_random_idx(idx: usize) -> usize{
    if idx < CHARSET.len(){
        idx
    } else{
        gen_random_idx(rand::random::<u8>() as usize)
    }
}

pub struct MerkleNode{}
impl MerkleNode{

    pub fn new() -> Self{
        MerkleNode {  }
    }

    pub fn calculate_root_hash(&mut self, chain: Vec<String>){

    } 
}

#[derive(Debug, Clone)]
pub enum RuntimeCode{
    Err(u8),
    Ok(u8),

}

struct CodePid{
    pub ramdb: HashMap<String, String>
}


/*  ----------------------------------------------------------------------
    implementing a dynamic type handler for structs and enums using traits
    ----------------------------------------------------------------------
*/
trait TypeTrait{
    type Value; // this can be the implementor type

    /* 
        we can use the lifetime of self in struct and trait methods 
        to return pointer since the self is valid as long as the object 
        itself is valid during the execution of the app
    */
    fn get_data(&self) -> Self::Value;
    fn get_ctx_data(&self, ctx: Self::Value) -> Self;
    fn fill_buffer(&mut self) -> &[u8];
}

impl TypeTrait for CodePid{
    type Value = Self; // the CodePid structure

    fn fill_buffer(&mut self) -> &[u8] {
        todo!()
    }

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        todo!()
    }

    fn get_data(&self) -> Self::Value {
        todo!()
    }
}

impl TypeTrait for MerkleNode{
    
    type Value = std::sync::Arc<tokio::sync::Mutex<HashMap<u32, String>>>;

    fn get_data(&self) -> Self::Value {
        
        let mutexed_data = std::sync::Arc::new(
            tokio::sync::Mutex::new(
                HashMap::new()
            )
        );
        mutexed_data
    }

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        todo!()
    }

    fn fill_buffer(&mut self) -> &[u8] {
        todo!()
    }
}

struct Streamer;
struct Context<T>{data: T}
impl TypeTrait for Streamer{ // polymorphism
    
    type Value = Context<Self>; /* Context data is of type Streamer */

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        ctx.data
    }

    fn get_data(&self) -> Self::Value {
        todo!()
    }

    fn fill_buffer(&mut self) -> &[u8] {
        todo!()
    }

}

impl TypeTrait for RuntimeCode{
    
    type Value = std::sync::Arc<tokio::sync::Mutex<String>>;
    
    fn get_data(&self) -> Self::Value {
        
        let mutexed_data = std::sync::Arc::new(
            tokio::sync::Mutex::new(
                String::from("")
            )
        );
        mutexed_data

    }

    fn get_ctx_data(&self, ctx: Self::Value) -> Self {
        todo!()
    }

    fn fill_buffer(&mut self) -> &[u8] {
        todo!()
    }
}

pub trait NodeReceptor{
    type InnerReceptor;
    fn get_inner_receptor(&self) -> Self::InnerReceptor;
}

pub trait Activation<C>: Send + Sync + 'static + Clone + Default{
    type Acivator;
}

impl<C> Activation<C> for &'static [u8]{
    type Acivator = &'static [u8];
}

#[derive(Default)]
pub struct Synapse<A>{id: A}

#[derive(Default)]
pub struct Neuron<A=u8>{
    pub data: Option<Synapse<A>>,
}

/* 
    this must be implemented for Neuron<Synapse<A>>
    to be able to call get_inner_receptor() method
*/
impl<A: Default> NodeReceptor for Neuron<Synapse<A>>
where Self: Clone + Send + Sync + 'static + Activation<String>, 
<Self as Activation<String>>::Acivator: Default{

    type InnerReceptor = Synapse<A>;
    fn get_inner_receptor(&self) -> Self::InnerReceptor {
        let id: A = Default::default();
        Synapse{
            id,
        }
    }
}

/* 
    this must be implemented for Neuron<String>
    to be able to call get_inner_receptor() method
*/
impl NodeReceptor for Neuron<String>{

    type InnerReceptor = Synapse<String>;
    fn get_inner_receptor(&self) -> Self::InnerReceptor {
        Synapse{
            id: String::from(""),
        }
    }
}

/* 
    this must be implemented for Neuron<A>
    to be able to call get_inner_receptor() method
*/
impl NodeReceptor for Neuron<u8>{

    type InnerReceptor = Synapse<u8>;
    fn get_inner_receptor(&self) -> Self::InnerReceptor {
        Synapse{
            id: 0,
        }
    }
}

pub fn fire<'valid, N, T: 'valid + NodeReceptor>(cmd: N, cmd_receptor: impl NodeReceptor) 
    -> <N as NodeReceptor>::InnerReceptor // or T::InnerReceptor
    where N: Send + Sync + 'static + Clone + NodeReceptor + ?Sized, 
    T: NodeReceptor, T::InnerReceptor: Send + Clone,
    /* casting generic N to NodeReceptor trait to access the InnerReceptor gat */
    <N as NodeReceptor>::InnerReceptor: Send + Sync + 'static{

    // with pointer we can borrow the type to prevent from moving and 
    // makes the type sizable at compile time by storing the address of 
    // none determined size of it inside the stack like str and []
    // box is sized with the size of its content allocated on the heap
    trait Test{}
    struct Neuronam{}
    let name = Neuronam{};
    impl Test for Neuronam{}
    let trait_name = &name as &dyn Test;
    struct AnotherNeuronam<T: Test, F> where F: FnOnce() -> (){
        pub data: T,
        pub new_data: F
    }
    impl<V: Test, T> AnotherNeuronam<V, T> where T: FnOnce() -> (){
        fn get_data(param: impl FnMut() -> ()) -> impl FnMut() 
            -> std::pin::Pin<Box<dyn std::future::Future<Output=String> + Send + Sync + 'static>>{
            ||{
                Box::pin(async move{
                    String::from("")
                })
            }
        }
        fn get_func() -> fn() -> String{
            fn get_name() -> String{
                String::from("")
            }
            get_name
        }
        }
    let another_name = AnotherNeuronam{data: name, new_data: ||{}};

    let cls = |func: fn() -> String|{
        func()
    };
    fn execute() -> String{
        String::from("wildonion")
    }
    cls(execute);

    let cls = ||{};
    let casted = &cls as &dyn Fn() -> (); // casting the closure to an Fn trait
    let name = (
        |name: String| -> String{
            name
        }
    )(String::from(""));
    
    enum Packet{
        Http{header: String},
        Tcp{size: usize}, // the size of the incoming buffer
        Snowflake{id: String}
    }
    let packet = Packet::Http { header: String::from("") };
    if let Packet::Http { header } = packet{
        println!("packet header bytes => {header:}");
    }

    enum UserName{
        Age,
        Id,
        Snowflake{id: String}
    }
    let enuminstance = (Packet::Tcp{size: 0 as usize}, Packet::Http { header: String::from("http header") });
    let res = match enuminstance{
        (Packet::Tcp { size: tcpsize }, Packet::Http{ header: httpheader }) | 
        (Packet::Http{ header: httpheader }, Packet::Tcp { size: tcpsize }) => {},
        (_, Packet::Snowflake{id: sid}) => if !sid.is_empty(){},
        _ => {}
    };

    /*  
        note that if we want to call get_inner_receptor() method
        on an instance of Neuron, the NodeReceptor trait must be
        implemented for every generic type in Neuron struct separately
        like:
            impl NodeReceptor for Neuron<String>{}
            impl NodeReceptor for Neuron<u8>{}
            impl NodeReceptor for Neuron<Synapse<A>>{}
    */
    let neuron = cmd;
    let neuron_ = Neuron::<String>::default();
    
    cmd_receptor.get_inner_receptor();
    neuron.get_inner_receptor()
    // neuron_.get_inner_receptor()
    
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum ActionType{
    #[default]
    A1
} 
type Method = fn() -> i32;
fn run<'lifetime>(param: impl Fn() -> ActionType, method: &'lifetime Method)
// bounding generic Method to traits and lifetimes
where Method: Send + Sync + 'static{}
fn execute<'f, F>(param: &'f mut F) -> () 
// bounding generic F to closure, lifetimes and other traits
where F: Fn() -> ActionType + Send + Sync + 'static{}

// bounding generic to traits and lifetiems
// async trait fn run in multithread env using #[trait_variant::make(TraitNameSend: Send)]
// bounding trait method only to traits like TraitName::foo(): Send + Sync
// return trait from method using -> impl TraitName
// trait as method param like param: impl TraitName
// trait as struct field like pub data: F (where F: TraitName) or pub data: Box<dyn TraitName> 
// casting generic to trait like &N as &dyn TraitName or N as TraitName
// bounding trait gat to traits like <N as TraitName>::AssetInfo: Send + Sync
// bounding the return type of closure trait to traits like where F: FnOnce() -> R + Send + Sync + 'static
trait Interface: Send + Sync + 'static{}
struct Instance{}
impl Interface for Instance{}
impl Interface for (){}
type BoxedTrait = Box<dyn FnOnce() -> ()>;
struct Test<R, F: Send + Sync + 'static + Clone + Default> 
    where F: FnOnce() -> R + Send + Sync + 'static, 
        R: Send + Sync + 'static{
    pub data: F,
    pub another_data: BoxedTrait
}
fn trait_as_ret_and_param_type(param: &mut impl FnOnce() -> ()) -> impl FnOnce() -> (){ ||{} }
fn trait_as_ret_and_param_type1(param_instance: &mut impl Interface) -> impl FnOnce() -> (){ ||{} }
fn trait_as_ret_type(instance_type: Instance) -> impl Interface{ instance_type }
fn trait_as_ret_type_1(instance_type: Instance) -> impl Interface{ () }
fn trait_as_param_type(param: impl FnOnce() -> ()){

    trait AuthExt{}
    #[derive(Clone)]
    struct Auth{}
    impl AuthExt for Auth{}
    impl Auth{
        fn get_trait(&self) -> &(dyn AuthExt + Send + Sync + 'static){
            let t = self as &dyn AuthExt; // use casting
            t 
            // &Self{}
        }
        fn get_trait1(&self) -> impl AuthExt{
            Self{}
        }
        fn get_trait2(&self) -> Box<dyn AuthExt>{
            let t = Box::new(self.clone());
            t 
        }
        pub async fn ret_cls(f: impl Fn(String) -> String) -> impl Fn(String) -> String{
            let cls = |name: String|{
                String::from("")
            };
            cls
        }
    }
    let inst = Auth{};
    let get_trait = inst.get_trait();

}


// C must be send sync to be share between threads safely
impl<F: Interface + Clone, C: Send + Sync + 'static + Unpin + Sized + FnOnce() -> String> Interface for UserInfo<C, F>{}
struct UserInfo<C: Send + Sync + 'static, F: Clone> where 
    F: Interface, 
    C: FnOnce() -> String{
    data: F,
    __data: C,
    _data: Box<dyn Interface>,
}
impl<F: Interface + Clone, C: Send + Sync + 'static + Unpin + Sized + FnOnce() -> String> UserInfo<C, F>{
    fn set_data(cls: impl FnOnce() -> String, clstopass: C, f: F) -> impl Interface{
        
        struct ExecuteMe;
        struct MessageMe;
        trait ExecuteMeExt<A, B>{
            type Result;
        }
        impl ExecuteMeExt<MessageMe, String> for ExecuteMe 
            where String: Send, MessageMe: Send + Sync{
            type Result = MessageMe;
        }
        
        Self{
            data: f,
            __data: clstopass,
            _data: Box::new(
                ()
            ),
        }
    }
}

struct SizeableImage{
    size: (u16, u16)   
}
impl Into<SizeableImage> for String{
    fn into(self) -> SizeableImage { // self refers to the String cause we're implementing this for String
        let mut splitted_size = self.split(",");
        let width = splitted_size.next().unwrap();
        let height = splitted_size.next().unwrap();
        SizeableImage{
            size: (width.parse::<u16>().unwrap(), height.parse::<u16>().unwrap()) 
        }
    }
}
fn construct_image<VALUE>(size: VALUE) where VALUE: Into<SizeableImage>{}

struct ErrorHandler<E> where E: std::error::Error{
    cause: Box<dyn std::error::Error>, // any type could causes the error at runtime, Error trait is implemented for that
    err: E
}
#[derive(Debug)]
struct ErrorItself{}
impl std::error::Error for ErrorItself{}
impl std::fmt::Display for ErrorItself{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
// let err = ErrorHandler{
//     cause: Box::new(ErrorItself{}), // return the error on the heap to move it around to implement for other types
//     err: ErrorItself{}
// };


async fn ltg(){

    // C must be ?Sized since its size can't be known at compile time
    // its can be either &[] or any type
    struct Gene<'r, C: ?Sized>{ 
        pub chromosemes: &'r C,
    }

    let gene = Gene::<'_, [u8]>{
        chromosemes: &[0, 255]
    };
    
    impl std::fmt::Display for ClientError{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }
    #[derive(Debug)]
    struct ClientError{}
    impl std::error::Error for ClientError{}
    let boxed_error: Box<dyn std::error::Error + Send + Sync + 'static> = Box::new(ClientError{});
    
    // traits
    //     - method param
    //     - return type
    //     - bound to generic and generic would be the type
    //     - cast the generic into a triat then bound the trait gat to other traits 
    //     - put them in box
    type ClsMe = Box<dyn FnOnce() -> ()>;
    trait NewTrait: Clone + FnOnce() -> (){} // if we want to implement NewTrait for the Fancy all the supertraits must be implemented for Fancy
    let cls = Box::new(||{});
    let cls_ = Box::pin(async move{}); // for future we must pin them
    struct Fancy<A> where A: Copy{name: ClsMe, age: A, fut: std::pin::Pin<Box<dyn futures::Future<Output=()>>>}
    let fancy = Fancy::<u8>{name: cls, age: 23, fut: cls_};
    impl<A: Copy> Fancy<A>{
        fn get_param(param: impl FnOnce() -> ()) -> impl Clone{
            String::from("") // we can return String in here since it implements Clone
        } 
    }

    #[derive(Debug)]
    struct CustomError{data: u8}
    impl std::fmt::Display for CustomError{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    
    impl std::error::Error for CustomError{} // in order to return an instance of CustomError the Error trait must be implemented for it so we can return the instance inside a boxed type
    // ----------------------------------------------------
    //              boxing traits be like: 
    // ----------------------------------------------------
    // (putting them on the heap and return an smart pointer with valid lifetime to move the around safely as an object)
    let boxed_cls: Box<dyn FnOnce() -> () + Send + Sync + 'static> = Box::new(||{});
    // boxing the Error trait allows us to handle any possible runtime errors 
    // reason of putting the trait inside the box is because we don't know the 
    // exact type of the object that caused the error and by putting it inside 
    // the box we're converting it into a safe object cause traits are not sized
    // at compile time their size depends on the implementor at runtime smt that 
    // implements the trait, if we don't want to box it we can use it as the return
    // type of the method but it needs to be implemented for the exact object that
    // may causes the error at runtime and we should return an instance of that 
    // type which implements the Error trait already, with this approach there is
    // no need to put the trait inside the box cause we know the exact type of
    // object that may causes the error and the syntax be like => ... -> impl Error{}
    let boxed_err: Box<dyn std::error::Error + Send + Sync + 'static> = Box::new(CustomError{data: 0}); // the instance of the implementor must be passed  - boxing trait to move them around as an object 
    // to move the future objects around we should pin them (the mutable pointer) into the ram 
    // to prevent them from moving by the compiler at runtime sine we may want to solve them
    // in other scopes and threads hence we must have their previous location inside the ram
    // to put .await on them
    let boxed_fut: std::pin::Pin<Box<dyn futures::Future<Output=String>>> = Box::pin(async move{String::from("")}); 
    let mut pinned_boxed_fut = Box::pin(async move{}); // pinning the boxed future to move it around for future solvation
    { // now we can await on the future in other scopes
        // await on the mutable pointer of the future cause we want to await on pinned_boxed_fut in later scopes
        // we can do this cause we've pinned the boxed future (pointer to future) on the ram which allows us to 
        // move it safely between scopes and threads
        (&mut pinned_boxed_fut).await; 
    }
    pinned_boxed_fut.await; // solve the future itself


    type ActorCls = Box<dyn FnOnce(fn() -> String) -> ()>;
    type PinnedBoxedFut = std::pin::Pin<Box<dyn futures::Future<Output=String>>>; // pinning the boxed future will be used to move the future around other scopes cause they can't move safely and we must kinda convert them into an object to move them
    pub struct GenericActor<'p, ActorCls: Clone, B, F> 
        where ActorCls: Send + Sync + 'static, 
        B: FnMut() -> fn() -> (),
        F: futures::Future<Output=String>{
        pub actor_cls: ActorCls,
        pub cls: B,
        pub fut: F,
        pub pinned: PinnedBoxedFut, // we can solve this later by putting .await on pinned field
        pub db: std::pin::Pin<&'p mut HashMap<String, String>> // pinning the mutable pointer of the map into the ram to move it safely between scopes without having changes in its location by the compiler
    }
}

fn serding(){
    
    #[derive(Serialize, Deserialize, Debug)]
    struct DataBucket{data: String, age: i32}
    let instance = DataBucket{data: String::from("wildonion"), age: 27};
    ///// encoding
    let instance_bytes = serde_json::to_vec(&instance);
    let instance_json_string = serde_json::to_string_pretty(&instance);
    let instance_str = serde_json::to_string(&instance);
    let isntance_json_value = serde_json::to_value(&instance);
    let instance_json_bytes = serde_json::to_vec_pretty(&instance);
    let instance_hex = hex::encode(&instance_bytes.as_ref().unwrap());
    ///// decoding
    let instance_from_bytes = serde_json::from_slice::<DataBucket>(&instance_bytes.as_ref().unwrap());
    let instance_from_json_string = serde_json::from_str::<DataBucket>(&instance_json_string.unwrap());
    let instance_from_str = serde_json::from_str::<DataBucket>(&instance_str.unwrap());
    let isntance_from_json_value = serde_json::from_value::<DataBucket>(isntance_json_value.unwrap());
    let instance_from_hex = hex::decode(instance_hex.clone()).unwrap();
    let instance_from_hex_vector_using_serde = serde_json::from_slice::<DataBucket>(&instance_from_hex);
    let instance_from_hex_vector_using_stdstr = std::str::from_utf8(&instance_from_hex);
    let instance_from_vector_using_stdstr = std::str::from_utf8(&instance_bytes.as_ref().unwrap());
    
    println!(">>>>>>> instance_hex {:?}", instance_hex);
    println!(">>>>>>> instance_from_bytes {:?}", instance_from_bytes.as_ref().unwrap());
    println!(">>>>>>> instance_from_json_string {:?}", instance_from_json_string.unwrap());
    println!(">>>>>>> instance_from_str {:?}", instance_from_str.unwrap());
    println!(">>>>>>> isntance_from_json_value {:?}", isntance_from_json_value.unwrap());
    println!(">>>>>>> instance_from_hex_vector_using_serde {:?}", instance_from_hex_vector_using_serde.unwrap());
    println!(">>>>>>> instance_from_vector_using_stdstr {:?}", instance_from_vector_using_stdstr.unwrap());
    println!(">>>>>>> instance_from_hex_vector_using_stdstr {:?}", instance_from_hex_vector_using_stdstr.unwrap());

}

// convert any generic of Vec into a generic slice by leaking and consuming the 
// memory of the vector to return an static reference to the leacked part since 
// that part will never be freed until the lifetime of the app
pub fn vector_slice<T>(s: Vec<T>) -> &'static [T]{
    Box::leak(s.into_boxed_slice())
}

fn but_the_point_is(){

    type Ret = &'static str;
    fn add(num1: Ret, num2: Ret) -> Ret where Ret: Send{
        for ch in num2.chars(){
            num1.to_string().push(ch);
        }
        let static_str = helpers::misc::string_to_static_str(num1.to_string());
        static_str
    }

    let addfunc: fn(&'static str, &'static str) -> &'static str = add;
    let res = addfunc("wild", "onion");

    let name = String::from("");
    let mut pname = &name;
    let mut anotehr_pname = &String::from("new content");
    println!("pname points to location of name : {:p}", pname);
    println!("anotehr_pname points to location of name : {:p}", anotehr_pname);
    pname = anotehr_pname;
    println!("pname points to location of anotehr_pname : {:p}", pname);
    println!("pname content : {:?}", pname);

    #[derive(Default, Debug)]
    struct User{
        name: String,
        age: u8,
    }

    let mut user = User::default(); // there is no null or zero pointer in rust thus the user must be initialized

    let mut mutpuser = &mut user; // mutating mutpuser mutates the user too
    println!("user address: {:p}", mutpuser); // contains user address
    println!("mutpuser address itself: {:p}", &mutpuser); // contains user address
    mut_user(mutpuser);

    fn mut_user(mut user: &mut User){ // passing by mutable pointer or ref to avoid moving

        // mutating the user pointer with new value which contains the user address
        // this makes an update to user instance too, can be viewable outside of the method
        println!("before mutating with pointer: {:#?}", user);
        user.name = "erfan".to_string(); // no need to dereference it since we're mutating only one field
        println!("after mutating with pointer: {:#?}", user);
        // or
        println!("before derefing: {:p}", user); // same as `contains user address`
        let mut binding = User{
            name: String::from("wildonion"),
            age: 0
        };
        // updating pointer which has the user instance value with a new binding by dereferencing pointer
        // note that we're not binding the new instance into the pointer completely cause by dereferencing
        // the underlying data will be changed
        *user = binding; // dereferencing the pointer to mutate it with new binding 
        println!("user after derefing: {:#?}", user);
        println!("user address after derefing: {:p}", user); // same as `contains user address`

    }

    // println!("out after mutating with pointer: {:#?}", user);
    let mut binding = User{
        name: String::from("wildonion"),
        age: 0
    };
    println!("mutpuser address itself: {:p}", &mutpuser); // contains user address
    println!("mutpuser contains address before binding: {:p}", mutpuser); // same as `contains user address`
    // binding a complete new instance to mutpuser, causes to point to new location
    mutpuser = &mut binding;
    // the address of mutpuser will be changed and points to new binding instance address
    println!("mutpuser contains address after binding: {:p}", mutpuser);
    println!("mutpuser address itself: {:p}", &mutpuser); // contains user address

    // we're getting a mutable pointer to an in place User instance
    // the in place instance however will be dropped after initialization
    // and its ownership transferred into mutpuser, Rust won't allow us to
    // do so cause a pointer remains after dropping the in place instance
    // which is and invalid pointer, we must use a binding to create a longer
    // lifetime of the User instance then borrow it mutably
    // mutpuser = &mut User{
    //     name: String::from(""),
    //     age: 0
    // }; // ERROR: temporary value is freed at the end of this statement

    // SOLUTION: using a `let` binding to create a longer lived value
    // let binding = User{
    //     name: String::from("wildonion"),
    //     age: 0
    // };
    // *mutpuser = binding;


    // let puser = &user;
    // println!("user address (puser): {:p} ", puser); // contains the address of user
    // let anotherpuser = puser;

    // println!("user address (anotherpointer): {:p} ", anotherpuser); // also contains the address of user

    // println!("pointer address: {:p} ", &puser); // the address of the puser pointer itself
    // println!("anotherpointer address: {:p} ", &anotherpuser); // the address of the puser pointer itself

    // user address (puser): 0x7ffea5896328
    // user address (anotherpointer): 0x7ffea5896328
    // pointer address: 0x7ffea5896348
    // anotherpointer address: 0x7ffea5896390


    let users = (0..10)
        .into_iter()
        .map(|_|{
            User::default()
        })
        .collect::<Vec<User>>();
    let slice_is_faster = &users;
    fn get_users(users: &[User]) -> (&'static [User], Vec<User>){
        // lifetime of users ends up here in this function 
        // and can't be as static accordingly can't be return 
        // from function
        let users_vec = users.to_vec();
        let static_users = vector_slice(users_vec.clone());
        (static_users, users_vec)
    }

    trait Interface{
        type This;
        fn getName(&mut self) -> &Self;
    }
    #[derive(Debug, Default, Clone)]
    struct UserPl{
        Name: String,
        Age: u8,
        IsAdmin: bool,
    }
    impl Interface for UserPl{ // unlike Go Interface in Rust will be implemented for both pointer and none pointer instances
        type This = Self;
        // trait and interface methods
        fn getName(&mut self) -> &Self { // we can return ref since the pointer is valid as long as instance is valid
            if self.Name == "oniontori"{
                self.Name = String::from("wildonion");
            }
            self
        }
    }
    trait ObjectSafeTrait{}
    impl ObjectSafeTrait for (){}
    let mut user = UserPl{Name: String::from("oniontori"), Age: 28, IsAdmin: true};
    let trait_object: Box<dyn ObjectSafeTrait> = Box::new(());
    let mut mutpuser = &mut user;
    mutpuser.getName(); // mutating the Name field of the user instance using the Interface trait and its mutable pointer
    // println!("user is changed {:?}", user); // the user is changed using its mutable pointer
    mutpuser.Name = String::from("change to anything else again");
    println!("user is changed {:?}", user);
    // println!("mutpuser is changed {:?}", mutpuser); // the mutpuser is changed also

    type LargeUInt = u128;
    type Func<A = UserPl, R> = fn(A) -> R; // A has a default type param
    let cls = |num: LargeUInt|{
        String::from("")
    };
    // `impl Trait` only allowed in function and inherent method argument 
    // and return types, not in variable bindings
    // let closure: impl Fn(u16) -> String = cls;

    #[derive(Default, Debug)]
    struct Player<'v, G: Default + Send + Sync + 'static, F> 
        where F: FnMut(Box<Player<G, F>>) 
            -> Result<
                std::pin::Pin<Box<dyn futures::Future<Output=&'v [u8]>>>, 
                Box<dyn std::error::Error + Send + Sync + 'static>> + Default{
        board: Vec<&'v [G]>,
        cls: F
    }

    trait UserExt<G, F>: Default{
        type Context;
        fn getCode() -> String;
    }
    impl<'valid, G: Default + Send + Sync + 'static, 
        F: FnMut(Box<Player<G, F>>) 
        -> Result<
            std::pin::Pin<Box<dyn futures::Future<Output=&'valid [u8]>>>, 
            Box<dyn std::error::Error + Send + Sync + 'static>> + Default
            > UserExt<G, F> for Player<'valid, G, F>{
        type Context = Player<'valid, G, F>;
        fn getCode() -> String {
            String::from("return value")
        }
    }
}