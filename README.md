# Nature

Nature is a __business process choreographer__, she decouple business data from technology such as __service__, __Interface__ etc.,
and take care of your data in consistence state, 
but you need to give the data to Nature first.
 
Nature like a proxy between services, and make the service easy to develop and maintenance.

Nature now is in very earlier stage. 

## Pain pot
![Alt text](doc/img/幻灯片4.jpg)
Communication between services is unreliable, 
we should do a lot of work for retrying and idempotent, 
but this is a complex, repetitive and hard task to do.

![Alt text](doc/img/幻灯片5.jpg)
Service is the owner of data, and control the entry for the data. 
If you want to change the date you must chang service first, 
but service is heavy to develop and deploy.

![Alt text](doc/img/幻灯片6.jpg)
We can't manage data directly, because it under service. 
managing services should be never the key parts to achieve our administrative purposes，
so there are a lot of redundant service manage works to do. 

## Concepts
![Alt text](doc/img/幻灯片8.jpg)
Like 7 Layers of the OSI Model, we must put a problem into a layer, 
I named it here `Thing`.
It let you to shrink it's range and make it easy to study, this is why I call it `Thing` but not layer.
A `Thing` may have many points and `Relation` may exits between points.
The `Thing` and point are __Static__, once they are created they can't be changed.

Relative to `Thing` and point, `Relation` is __Instantaneous__, either it happened or it has not.
so relationships are very time-dependent，`Relation`'s can change over time.
Maybe __Event__ is a good word to express the meaning. but event has a random meaning,
yet relationships emphasize specific behaviors of specific participants,
so I choice `Relation`

![Alt text](doc/img/幻灯片9.jpg)
Let me say more about `Thing` and point. 
A `Thing`may have points, and points maybe another `Thing` for sublayer points.
So point and `Thing` are conceptually equivalent.
I think `Thing` feel more meaningful than point. so I use `Thing` to express `Thing` and or point.

An amazing thing is that `Thing` contains __Hierarchy__. 
The benefit of this is simplify Nature and give freedom management ability to human. 
`Nature` don't care of __Hierarchy__ but need to identify every `Thing`, 
this is a easy way to satisfy `Nature` and human management simultaneously.

![Alt text](doc/img/幻灯片10.jpg)
`Thing` is only a define, at run-time you must bind an `Instance` to `Thing`.
If a `Relation` exists between two `Thing`'s, 
`Nature` will give upstream `Instasnce` to a converter which is binding to the certain `Relation` and it is at anywhere outside the `Nature`.
Then the converter generate new `Instance`s for the downstream `Thing`.
    
Here the `Relation` made a great job: It locked the __Geographic and Time Coordinates__, 
they are important for `Natrue` to retry when failed the execution. 

![Alt text](doc/img/幻灯片11.jpg)

Converter is just one simple function, and the converter achieve __only one__ goal for the `Nature`.
Converter do no need to know all the other goals in `Nature`,
`Nature` will take care of them by call the converters recursively.

![Alt text](doc/img/幻灯片12.jpg)
It is very easy to organize the goals in `Nature`.
There is no __Flow Control__, so there is no  __loop__, no __if__ to use to do __Business Process orchestration__ in `Nature`.

down-thing to __select__ upper-thing but not upper-thing to __control__ down-things.
because direct upper-things are more less down-things and __control__ will more complex than __select__..
This mechanism greatly simplifies the process.

Another key point is `Instance` bind to `Relation` but not to a __technical__ `Interface`.
It's purl business dependence,it has nothing to do with technology.
Then `Relation` made a __very short process__, only one step,
but the __simple relation__ can form any complex and large network.

There are good news for developers too, __short process__ will be easy to develop, 
and `Nature` try to run the __short process__ in parallel.

![Alt text](doc/img/幻灯片13.jpg)

`Nature` only insert `Instance` to db, no __deletion__ no __update__.
`Nature` use __state version__ to express any chang on an `instance` and not all `Thing` have states.
This feature let `Nature` run converter equally for any times, tt is __idempotent__.

## Significance
![Alt text](doc/img/幻灯片15.jpg)

![Alt text](doc/img/幻灯片16.jpg)

![Alt text](doc/img/幻灯片17.jpg)

## Why need Nature

To make data consistence is a not easy work for engineers, most of all the data transferred between services. We need check, redo and make sure the target services are idempotent, we waste much time and money on that things.

Nature organize all the data into a web, you can see every data's flow in real-time. the most important point is, Nature let you orchestrate the date flow over the services, no technology but pure business. Nature will be a **Business Command Center**!
 
## Conception

Nature is a abstract from the real nature. It is a dynamic system which changes things and transforms things incessantly over time. 

### Things

Things are need to be changed or transformed. this is the aim why we need to program.

#### type

#### instance

status version

### transform

It controls how to change or transform the **Things**

## What should it do

## How to use

### before to use

#### ID generator

**Nature** use **Thing**'s MD5 hash code as id by default. 

Maybe you need a **Distribute ID Generator** like [Twitter's snowflake](https://github.com/twitter/snowflake/releases/tag/snowflake-2010) for storing large data in a distribute DB.

