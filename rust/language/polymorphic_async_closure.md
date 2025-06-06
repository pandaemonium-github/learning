# Using polymorphic async closures in rust

${toc}

(https://www.youtube.com/watch?v=ukyz5haC-N0)

## When do we want a collection of asynchfronous closures

example: message dispatcher: ingests one or more messages, and forwards it to 0 or more receivers
UI systems often use async callback functions
web servers can u async callback functions for requerst processing


## Message dispatcher

### Dynamic dispatch

Since callbacks have different signatures, we need dynamic dispatch. For this we need a trait object.
Trait objects let us dynamically dispatch to implementations of a known trait for an unknown type.

We can use either: 
- [Box, Arc, Rc]::<dyn Trait>
- dyn Trait

This uses a couple of informations: a pointer to the data, and a pointer to the vtable. This is referred to as fat pointer:
``` rust
struct FatPointer {
    data: *const c_void,
    vtable: *const c_void
}
```

The vtable contains pointers to each trait method implementation.
The trait method implementation is called, passing the reference to the corresponding data: &self.

### Async closure

An async closure is something that returns a future:
``` rust
type MyAsyncClosure = impl Fn() -> /* impl Future<Output=()>*/
```

The async block created by the closure suffers the same problem as for event dispatcher: the signature differs depending on the type of the future:
``` rust
let mut fut: impl Future<Output=i32>+Sized = async {666};
fut = async{666};

let mut = async {666};
         -------------
         |
         the expected `async` block
         expected due to this value
fut = async {666};
      ^^^^^^^^^^^^ expected `async` block, found a different `async` block
```

Closures must return futures as trait objects.

When trying to return a box dyn trait as future type, the compiler complains that the trait `Unpin` is not implemented for `dyn futures::Future<Output = ()>`
This is because self referential structs cannot freely move in memory.

To solve this problem rust has the Unpin auto trait.
It is implemented for all standard types.
An Unpin type is guaranteed to have no constraints on moving values of it in memory.

Self referenced structs must use the PhantomPinned attribute to make them unpinned: 
``` rust
struct SelfReferentialStruct {
    a: i32,
    pointer_to_a: *const i32,
    _marker: std::marker::PhantomPinned
}
```

The pin struct is used to wrap a pointer type:
``` rust
struct Pin<Ptr> {...}
```
Pin<T> guarantees that even if the type T is !Unpin, the pinned value will never move, so Pin<T:!Unpin> is still Unpin.

Pin/Unpin was invented to avoid problems with self-referential Futures.

``` rust
async fn use_name(name: &str) {
    println!("I'm referencing {name}");
}

async fn self_referencing_future() {
    let name: String = "John".to_string();
    use_name(&name).await;
    println!("Hello, {name}");
}
```

To solve our dispatching problem, we need to declare a pin box dyn future:
``` rust
async fn pin_async_closure(f: Box<dyn Fn() -> Pin<Box<dyn Future<Output=()>>>>) {
    f.await;
}
```

### Async message dispatcher

``` rust
struct SomeDispatcher {
    callbacks: Vec<Box<dyn Fn(String) -> Pin<Box<dyn Future<Output=u32>>>>,
}

impl SomeDispatcher {
    pub async fn dispatch (&self, mst: &str) {
        let mut dispatch_task_set: FutureUnordered<Pin<Box<...>>> = FutureUnordered::new();
        

        for callback: &Box<dyn Fn<...>> in selff;callbacks.iter() {
            dispatch_task_set.push(callback(mst.to_string()));
        }

        while let Some(output: u32) = dispatch_task_set.next().await {
            println!("dispatch finished with output: {output});
        }
}
```

When passing a value by reference to the dispatcher we have the following error:
```rust
dispatcher.register_callback(Box::new(move |msg:&str| {
    let name:STring = name.clone();
    Box::pin(x:async move {
        println!("The message to {name} is: {msg}");
    })
}));

dispatcher.register_callback(Box::new(move |msg| {
                                            --- return type of closure is std::pin::Pin<Box<(dyn futures::Future<Output = ()> + '2)>>
                                            | 
                                            has type `&'1 str`

    let name = name.clone();
    Box::pin(async move {
        println!("The message to {name} is: {msg}");
    })
____^returning this value requires that `'1` must outlive `'2`
```

Trait object lifetime elision uses the "default trait object lifetime bound"

In our case the lifetime for dyn future is 'static by default.

By using an anonymous lifetime constraint, we apply the normalize lifetime elision which bind the lifetime to the lifetime of the reference passed to the callback:
``` rust
Box<dyn Fn(&str) -> Pin<Box<dyn Future<Output=()> + '_>>>>,
```

If the function would have more than one reference, this trick no longer works since the compiler does not know which reference lifetime to use:
``` rust
Box<dyn Fn(&str, &str) -> Pin<Box<dyn Future<Output=()> + '_>>>>
```

This is where the concept of higher rank trait bounds comes into play.
``` rust
Box<dyn for<'a> Fn(&'a str, &'a str) -> Pin<Box<dyn Future<Output=()> + 'a>>>
```



