-----------------------------
interact with gem repo mmr.rs
-----------------------------
build dl models based on phases dataset using keras and https://github.com/wildonion/stem/
behavioural graph virtual machine built on top of each event's `phases` field inside the game 
for each player to match them for new game and rank them based on their in-game statuses, the 
match making rating or ranking (**MMR**) engine, on the other hand is is a weighted tree based 
suggestion engine that suggests players, events and other games and players based on their ranks 
earned using **GVM** during the game.

never return poiner from method just return vec or string but pass them in slice form to not to lose 
their ownerhsip cause can't return ref to a data owned by the method we have to either ret the type in 
its slice form with valid lifetime or return it as self.field cause self has longer lifetime also the 
memory allocation process in rust depends on the types' lifetimes means that every type in rust has a 
valid lifetime and as soon as the type gets moved into other scopes or threads or methods its lifetime 
will be dropped from the ram and will be owned by that scope in other words we can't move out of a type
or deref it if it's behind a shared ref or pointer also we can't have a type in two scopes at the same time
without passing a reference or a clone of that into the second scopes, we should either clone the type or 
borrow it and pass one of these to the second scope cause rust doesn't have gc to track the references 
came to the type and use that to destroy the type when it reaches the zero instead it's using lifetime 
conceptes which in single thread contexts we can use Rc to count the references of a type shared between 
scopes but in multithread contexts we must use Arc and Mutex or RwLock to share the ownership of the type 
between threads without having race conditions and deadlocks although rust forces us to use every type only 
once during the whole lifetime of the app which this manner prevents us from allocating extra space on the 
ram and deadlocks situation since the concept of ownership and borrowing is about sharing a type with 
reference or cloning to not to lose its ownership in future scopes, if we allocate something in a scope we 
can't move it out of that cause its being used inside that scopes, we have to borrow it or clone it or convert 
it to its owned type if it's a sliced form or pointer of a type, slices are just a representation of dynamic 
types with no dynamic allocation feature.

this is lle parallel based vm and game engine graph like DOM, an state manager 
like yew and redux with tree walking using (shared ref and mutably) using rc and 
arc weak and strong ref counting, shared ownership and interior mutability, based on 
actor and graph concepts so we have followers weighted tree to understand the 
relationship between peers to suggests events in a graph virtual machine by using 

lazy static global, mutexed, rwlocked, mpsc, rusty ltg pointers, slices, codec hash hex and file bytes,
ret &'validlifetime ref and trait as param like -> impl Trait as type from method and use them in method 
param like param: impl Trait from method also can't move type if it's behind pointer send sync static, 
shared ownership using Mutex and RwLock and RefCell, GlobalAlloc arena referene counting using Rc Arc, Box 
leaking, Pin, &mut type, cap, length, macros, Cow, Borrowed, ToOwned, Deref, &mut (ast, token stream), 
std::mem, generic, lifetimes, closures, traits, pointers and bytecode, .so and .elf bpf, wasm, bytes and 
hex serding and codec ops using borsh and serde, async trait and associative bounding Trait::method(): Send and 
?async and ?const, gen block, r3bl_rs_utils crate, read/write io traits, Box<dyn Trait>, as_ref(), unwrap(), clone() 
and Box stores data on the heap and contains an smart pointer with a valid lifetime to the underlying type, 
also the size of the boxed type is the size of the type itself, the value of the box can be caught 
by dereferencing the box

every type has its own lifetime and if it goes out of scope it'll be dropped from the ram and we can 
either borrow it or clone it since rust dones't have gc instead it has rc, refcell for single thread 
and arc and mutex for multithread reference counting and borrowing based on these concetps share ownership 
between threads using Arc by borrowing the ownership using pointers like & clone share ownership between 
scopes using Rc by  borrwoing the ownership using pointers like & and clone Rc is not safe to be used between 
threads but Arc can be used to share the type between multiple threads safely without having race conditions, 
also if we want to mutate an immutable type at runtime we must use RefCell which is a single threaded smart 
pointer and for mutating type between multiple threads we must use Mutex or RwLock to avoid deadlocks situations.

Single Thread    Multithread             Usage
Rc               --> Arc                 make the type shareable between scopes and threads
RefCell          --> RwLock || Mutex     make the type mutably safe at runtime in scopes and threads

everytime we try to move type betweeen scopes and threads without losing ownership we're taking a reference to that thread so:
    share and mutate the actual type using its &mut pointer in a single thread scope
    share and mutate the actual type using its Arc<RwLock<Type>> pointer in a multithread scope   

all ltgs in rust ::::: https://github.com/wildonion/rusty/blob/main/src/retbyref.rs#L17
zero copy        ::::: https://github.com/wildonion/uniXerr/blob/a30a9f02b02ec7980e03eb8e31049890930d9238/infra/valhalla/coiniXerr/src/schemas.rs#L1621C6-L1621C6
data collision   ::::: https://github.com/wildonion/uniXerr/blob/a30a9f02b02ec7980e03eb8e31049890930d9238/infra/valhalla/coiniXerr/src/utils.rs#L640 
near rules       ::::: https://github.com/wildonion/smarties/blob/main/contracts/near/NEAR.rules
solana rules     ::::: https://github.com/wildonion/solmarties/blob/main/SOLANA.rules
https://github.com/wildonion/uniXerr/blob/a30a9f02b02ec7980e03eb8e31049890930d9238/infra/valhalla/coiniXerr/src/schemas.rs#L1305
https://github.com/wildonion/uniXerr/blob/a30a9f02b02ec7980e03eb8e31049890930d9238/infra/valhalla/coiniXerr/src/schemas.rs#L1213
https://developerlife.com/2022/02/24/rust-non-binary-tree/#naive-approach-using-weak-and-strong-references
https://developerlife.com/2022/03/12/rust-redux/
https://bevyengine.org/learn/book/introduction/  
https://godotengine.org/
https://nannou.cc/
https://crates.io/crates/rg3d
https://amethyst.rs/
https://fyrox-book.github.io/introduction.html
https://www.youtube.com/watch?v=yq-msJOQ4nU
https://github.com/wildonion/cs-concepts
https://github.com/wildonion/rusty => all ltg codes
https://doc.rust-lang.org/nomicon/index.html
https://stackoverflow.com/questions/26271151/precise-memory-layout-control-in-rust
https://docs.rust-embedded.org/book/
https://crates.io/crates/hotham
https://developers.google.com/protocol-buffers/docs/encoding
https://capnproto.org/encoding.html
https://ethereum.org/nl/developers/docs/evm/
https://blog.subnetzero.io/post/building-language-vm-part-01/
https://rust-hosted-langs.github.io/book/
https://benkonz.github.io/building-a-brainfuck-compiler-with-rust-and-llvm/
https://opensource.com/article/19/3/rust-virtual-machine
https://medium.com/iridium-vm/so-you-want-to-build-a-language-vm-in-rust-part-09-15d90084002
https://medium.com/clevyio/using-rust-and-nom-to-create-an-open-source-programming-language-for-chatbots-12fe67582af5
https://cheats.rs/#behind-the-scenes
https://github.com/ethereum/evmone => compiled smart contract bytecode executes as a number of EVM opcodes
https://blog.logrocket.com/guide-using-arenas-rust/
https://zhauniarovich.com/post/2020/2020-12-closures-in-rust/
https://blog.cloudflare.com/pin-and-unpin-in-rust/
https://fasterthanli.me/articles/pin-and-suffering
https://stackoverflow.com/questions/2490912/what-are-pinned-objects
https://medium.com/tips-for-rust-developers/pin-276bed513fd1
https://users.rust-lang.org/t/expected-trait-object-dyn-fnonce-found-closure/56801/2
codec, virtual machine like move and evm with allocation concepts 
    - macro dsl
    - thread_local, actor id or address, std::alloc, jemalloc, bumpalo and r3bl_rs_utils arena as a global allocator
    - zero copy 
    - null pointer optimiser
    - unique storage key

________________
>>> concepts <<<
----------------
https://github.com/wildonion/gvm/wiki/Ownership-and-Borrowing-Rules
actix actor(acter.rs) worker pubsub concepts to build p2p app like ai based nft game market and 
graph(adjmat,gvm) (gvm) allocation engine with oauth2, noir, sui, mdp, mlx, dag, thiserror, 
tauri, yew, wasm(async,multithreaded like spacetimedb) and merkle tree concepts by building graphs using 
    - ctxruninterval to sub constantly in sub actor and loop{} to run the app constantly 
    - move trait as object safely by putting them inside the Box to send them on the heap
    - share ownership and break cycle of self-ref instead of moving using &mut, rc, refcell arc, mutex and mpsc 
        between scopes and threads cause rust move heap data by default and updates all the pointers after moving 
        but can't use them so it's better not to pass them by reference instead of cloning or moving 
    - !Unping data must be pinned at a fixed position in the ram so we can move them safely, since they got fixed in the ram thus their pointer won't be updated after moving and remain valid and the old one
    - don't move if it's behind a pointer and to move pointer the borrow must last and live long enough once it’s moved, 
    - can’t move pointer between different scopes unless the pointer lifetime is greater than the new scope lifetime after moving that's because a reference of a pinned value remains valid
    - pin the boxed future to move the future safely like awaiting on its mutable pointer cause awaits consumes it
    - use Box<dyn std::error::Error> to handle all possibel runtime errors for every type
    - use Result<(), impl Error> to return the exact type of error at runtime
    - can’t start actor inside tokio spawn or the context of tokio::main it must be inside actix_web::main context
    - can't start tokio tcp inside the main function body of the actix_web::main context
    - tokio::spawn() and mpsc for nodejs like async execution
    - wallexerr::ed25519_with_aes_signing for checksum, file digital signature, secure communication and zk logics
    - local based pubsub using mpsc, actixbroker and tcp based pubsub using libp2p and redis and grpc
    - tokio::tcp,channels::mpsc,spawn,time,select,arcmutex,rwlock,asynciotraits,while let Some ::: #[tokio::main]
    - redis::queue,stream,set,get,pubsub 
    - actix http request, stream and ws stream handler also tokio::* ::: #[actix_web::main]
    - ipfs mdp graph::libp2p::kademliadht,gossipsub,noise protocol,quic,tokio::tcp,p2pwebsocketwebrtc
    - serde_json::from/to, std::str::from_utf8, stream:Payload, payload:Multipart for while let some byte streaming 
    - graph with rc refcell in single thread context | graph with arc mutex in multithread context
    - gloabl static lazy arc muted box pin futdata => global allocator in multithread context
    - threadlocal with rc refcell => global allocator in single thread context
    - enum unique storage key, actor id, r3bl_rust_utils, jemalloc and bumpalo arena as global allocator
    - gvmllevm:str<->hex<->bytes<->base58/64,&mut,casting,ltg,boxpin,boxleacking,codec,simd,wallexerr
    - sharing (AppState: Send + Sync + 'static) between threads using mpsc
    - #[actix_web::main] context
        - http  ---> apis, web::Json, web::Path, Payload, Multipart
        - ws    ---> while let some streaming over Payload bytes
        - actor ---> redis and local borker pubsub actors
        - async ---> tokio::spawn(), Box::pin
    - #[tokio::main] context
        - tcp listeners with while let some streaming
        - spawn(acter.rs),channels::mpsc,select,time,arcmutex