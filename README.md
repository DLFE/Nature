# Nature

[English](README_EN.md)|中文

## Nature 时什么？

### 极简开发平台

Nature 是一个基于网络的适用于各种编程语音的**极简开发平台**。Nature 提供了**统一的数据存储模型**和**基于配置的业务流程控制**，业务的差异性更多以配置的形式体现而不是传统的一个个独立的代码系统，这样 Nature 就可以大幅度减少设计、开发相关的工作，如：

- 使用者一般不需要额外的数据库设计以及相应的代码支撑，Nature 的数据库模型可以适用于很多业务场景。
- 使用者一般不用考虑流程控制、时间调度、幂等、失败重试等问题，Nature 内置的流程控制提供了数据最终一致性保障。

Nature 是一种基于**数据驱动**、**面向业务**和**去中心化**的用于构建**大型系统的支撑平台**。Nature 能够有机的**将系统架构和业务架构整合到一起**，消除两者的隔阂，使系统能够更好的服务于业务。

Nature 使用下游"**自然选择**"上游的方式来替代传统的**上游控制下游**方式来构建系统，它将**集中控制的一对多**变为**去中心化的一对一**，从而将事务的复杂性大为降低，这可以大幅度精简业务模型，并满足业务快速迭代的需要，使得系统在不断**选择**中实现**进化**，这也是本项目名称的寓意所在。

Nature 可以**使业务对代码具有完全的控制能力**，不会受既有系统这样或那样的约束。你可以通过非编码的方式实现业务流程的控制，这会有两个方面的重要意义：一是更容易塑造你的系统，二是可以大幅度减少开发投入。

## 传统开发方式的问题

系统要实现业务目标，**需要一整套软件工程为之保驾护航**。出于人员，管理和时间成本的考虑，我们往往会做不同程度的裁剪，再结合各种开发模式，如敏捷，以使得质量和成本之间达到一个平衡。但即便是这样研发的成本依然高企，这是因为传统开发方式绕不开以下几个方面的问题：

- **传话游戏**：大型系统一般都需要经过需求、产品、设计、研发等多个不同职能的团队来协作，所以往往具有高延迟、低效等问题。目标在层层传递的过程中，还会有不同程度的损失，为了减少这些损失，我们一般还要引入质控环节，这些都是成本体现，环节越多成本越高。
- **弱控制性**：需求提出者大多不是开发人员，他的想法只能通过他人间接的体现在系统上，**不能直接控制**。对于系统这座冰山来讲，需求提出者也**仅仅能够对水面上的部分做一些指导**，并没有绝对的控制能力，水下部分更是没有发言权。
- **目标被绑架**：目标是用代码来体现的，然而业务想变，代码却变起来很难！**大系统里总有一些模块大家不敢动！是系统中永远的痛**
- **现实很骨感**：口号可以喊的非常清晰响亮，但当真正细化下来，并沉淀到实施细节中时，在业务、技术、成本等多种维度的夹击下，技术人员很难做出理想的系统。 在**快速迭代，持续交付**的信念支撑下，寄托于下次的修正。
- **目标迷失**：我们大部分的工作都是面向功能的，我们只是偶尔回头看看目标。天长日久，不自觉的就陷入了功能的漩涡，而**忘了自己真正想要的东西**。
- **非功能性需求**：对于一个复杂的业务系统而言，非功能性需求的开发难度和耗时要远大于功能性需求，如幂等、重试、一致性、稳定、可靠等，**这些都是吸金的黑洞**。

## Nature 的解决方式

传统开发方式所面临的问题，是本身所固有的，是无法通过完善自身来解决的。要想解决这些问题，需要使用全新的机制来面对，Nature 便提供了这样的一种机制。

Nature 是面向业务的，它对业务进行了高纬度的抽象，一切复杂的业务世界都可以用**业务对象**以及它们间的**关系**来表示。业务对象其实就是**业务目标**，而关系是可以让你一个接一个**实现这些目标的跳板**。关系不是方法，也不是功能，但却是功能的载体。它不告诉你怎么做，但却能告诉你需要什么。

### 业务对象的“目标”属性

宏观上讲，系统就是为了处理数据。显示器里看到的，系统间交互的都是数据。所以**系统为数据而生，数据就是系统的目标**。从微观上讲，一个方法输入的是数据，输出的也是数据。**方法存在的意义就是处理数据，数据就是方法的目标**。由此可见数据才是我们想要的东西，从这个意义上讲**数据就是目标**。业务对象是描述业务的数据，所以**业务对象是系统的灵魂**。业务对象管好了，目标也就管好了，而 Nature 就是为管理业务对象而生。

### 从代码中解放目标，使目标由隐性变为显性

在传统开发方式下，业务对象被代码所定义。在 Nature 中业务对象被称为`Meta`，是以配置的方式进行定义的。这是一种人性化的定义方式，不需要编程就可以操作。`Meta` 是数字化的，直接参与系统的运行，从而避免了下面问题的发生。

- 目标下达**不需要传话**了。一个人定义就好了，所有的人看到的都是同一个东西，从而避免了失真问题的产生，这样目标的传递、转译和管控成本都没有了。
- 目标**具有强控制性**。代码的身份由定义者变成了跟从者，不再有约束目标的资本，目标具有了真正**控制而不是指导**代码的能力，从而**不用担心被代码绑架**的问题发生。
- 细化工作和业务更迭只关心业务维度就好了，**技术性迭代将变得毫无意义**。

### 剥夺代码对业务流程的控制权，降低业务系统的复杂度。

传统业务系统之所以复杂，就是因为体系庞大。不管是技术的还是业务的，很多逻辑交织在一起，就像一团乱麻，环环相扣。Nature 则将这个聚合体拆分成了三个部分：技术的部分尽可能的由 Nature 自身来承担，功能实现留给开发人员，而把最重要的部分——流程控制留给了业务管理人员，这部分是配置化的，不需要编程就可以轻松掌控。这样就把一个复杂的系统简单化了，条理化了。

Nature 用`Relation`来表达业务流程控制。请放心，Nature 并没有把它变成类似于脚本的一种具有复杂度的新语音。 `Relation`只是说明两个`Meta` 间存在直接的关系，并且可由前一个`Meta`导出下一个`Meta`，仅此而已。**Nature 用 `Relation` 这种极简的方式来描述所有的业务流程控制，没有 if else, 没有loop 根本不需要难以沟通的程序员来帮忙**。

`Relation`为什么能够实现流程控制呢？其实原理很简单，`Relation`中的两个`Meta`一个是输入一个是输出，这不是一个**隐形的功能**吗？`Relation`里有个`执行器`属性用于完成这个使命，有了这个`执行器`业务流程控制就可以运转起来了。`Relation`  就像是火车轨道，而`执行器`就是轨道上的火车，**不需要方向盘就可以跑的非常准非常块**。在Nature里只有`执行器`是需要开发人员介入的，但你可以看出**程序员已经失去了实际控制的能力**。

有了`Relation` 的帮忙，下面的问题将迎刃而解。

- **目标迷失问题**：我们很难陷入功能陷阱中，`Relation`只是建立目标间的联系，你不用考虑功能实现问题。
- **非功能性需求**：`Relation`使得业务的流转变得统一，这种统一可以使得传输保障工作的多样性和复杂性大为简化，这样 Nature 就有能力为你打理好这方面的技术工作，程序员可以不用加班了。

## Nature 的价值

Nature 以一种自上而下的方式，实现业务对技术的**完全控制**（而非主导，主导存在妥协），直接且高效。并用一种及其简单的模型构建方法来帮助业务系统提炼核心职能，**使业务布局尽收眼底且不遗漏任何细节，系统越大效果越明显。**Nature 强迫业务人员拥有结构性思维，弱化功能性思维对系统带来的伤害。即便是设计人员主导。也容易形成一个可共同理解的界面，**摆脱了功能性驱动的魔咒**，避免了目标被淹没和迷失的风险。

Nature 只用 `Meta` 和 `Relation` 两种元素来表示复杂的业务世界，这个“**真实而实时**”的业务世界不再隐藏在代码底下，而是以清晰、实用、干净、纯粹、直白的方式服务于业务决策者，**可以使系统少走弯路，大幅度减少中间成本的支出。**

Nature 使得目标与目标之间，目标与代码之间，代码与代码之间最低程度耦合。**用最少的规则来实现最大的灵活性**，非常有利于系统快速迭代，发挥不同组织的积极性以及协同促进。

**Nature 内置的基于分布式的稳定性、可扩展性、高并发性和高可用性，可以大幅度减少开发和维护支出**。

## 与其他系统的关系

### 消息系统

Nature 中的“选择”是从消息系统中借鉴而来的，有点类似于消息系统的发布与订阅，但 Nature 与消息系统存在着本质的不同。消息系统仅仅是一种技术形式，他允许开发者既是球员（功能开发）又是裁判员（目标数据定义）。其次消息是暂态的，消息与消息之间没有必然的相关性，消息系统的本质是解耦两个技术过程。而 Nature 是面向业务的，不关心技术形态，其中的数据是永恒的，数据和数据间存在强相关性。Nature 中的“选择”衔接的是两个目标而不是过程，衔接的是整个业务网中的一个一个的里程碑。

### `workflow` 和 `BPMN` 

从业务流程编排来看，Nature 与 `workflow` 和 `BPMN` 有一定的相似之处。其重要区别在于后者告诉我们**怎么做（功能性驱动）**，“怎么做”是非常复杂的，具象的，迭代多了我们的**目标可能会迷失**。而 Nature 告诉我**需要什么**，而“需要什么”是简单的但却是最为重要的更不会迷失，Nature 让我们**聚焦到真正重要的事情**上，并**让管理变得简单**。Nature 把怎么做交到了外部的 `执行器` 去处理，如何处理是不去管的。

### FaaS 和 Serverless

Nature 和 `执行器` 协作的方式实际上可以看做是 FaaS(Function as a Service) 或者 Serverless 的一种形式，但我不想使用这个概念，因为`FaaS` 是面向功能的，而Nature 是面向业务目标的。

### 数据库

Nature 的本质其实是一款数据管理产品，试图囊括各种业务数据，并处理好它们之间的关系。这种数据管理不同于传统数据库的数据管理，传统数据库的数据管理主要是面向技术的，或者说是面向系统的；而 Nature 的数据管理主要是面向业务的。或者说是面向企业的。

如果你看 Nature 数据表中的数据，会发现它们很像是`Key-Value`数据，你也会看到 数据之间可以通过 `from_key` 相互关联， 但却几乎无法使用数据库的 join 来关联查询。 Nature 借鉴了关系数据库的`关系`技术，用简单的一对一来解决一对多，多对多等复杂关系，但却与关系数据库中的关系有很大的不同。据库中的关系只是一个外键ID，而Nature 中的关系却有非常多的内含及丰富的数据控制机制。Nature 与数据库的区别在于：

- Nature 关注于业务对象的整体而数据库更关注于业务对象的细节。
- Nature 专注于驱动业务运转，而数据库更倾向于数据存储于检索。
- Nature 本身不具备存储能力，还需要借助具体的数据库产品来存储。

## 深入了解Nature

[示例及功能讲解](https://github.com/llxxbb/Nature-Demo)

[Nature 架构说明](doc/ZH/help/architecture.md)

[使用 Meta](doc/ZH/help/meta.md)

[使用 Relation](doc/ZH/help/relation.md)

[内置执行器](doc/ZH/help/built-in.md)

[一些业务情景的解决方法](doc/ZH/help/use-case.md)

## 注意

本系统还处于早期阶段，有不妥之处，还请多提建议。

