#Rationale:

#Objectives:

##Resolve knowledge dependency issues

    A particular topic should have its pre-requisite topics made clear. For example, the knowledge of derivatives and integrals is a pre-requisite to differential equations.

    As much as possible, dependency "loops" have to be avoided. This is particularly deadly and here's how it works: Topic A depends on a learner understanding topic B, but topic B depends on the learner understanding topic A. In essence, one has to understand either one of the topics to even be able to understand both and this defeats the purpose of the project. The way such a thing can be avoided is to first ask learner to use suspension of disbelief, or just declare something as a definition, and then the resulting theorem should justify said definition or explain why the suspension of disbelief was initially correct.

    A particular document should always be able to trace its path back to "first principles", which are either nested in physical observation like Newton's 2nd law or some definition agreed upon definition, for example definition of integers.

##Segmentize knowledge into small parts

    Document should be split into small modular pieces. This helps with recyclability. 

    In general documents should be attempted to be kept as small as possible. However, if a group of particular topics are really interrelated, they should be included in the same file. For example, it wouldn't make sense to have integral and differential continuity fluid mechanics split in different documents. Continuity integral form is built on top of differential continuity form, and in many instances, continuity integral refers to many variables of differential continuity. So there is some tradeoff in which knowledge should be grouped together and which could be split. If two topics are not too interrelated, they can be split.

    Doing this also has advantages particularly when collaborating. Files allow the writing of the archives to be more distributed, and makes it easier to do merges.

##Generalization and Abstraction
    
    Generalizations allow equations to become more 'powerful' in that they are able to describe a wider variety of cases.
    
    However, the more generalizations and abstractions are implemented on a particular topic, the harder it is to write, and the harder it is to understand for the inexperienced learner. 

##Enact Variable Naming Standardization
    
    Variables should be named. If a variable was never or defined, it shouldn't be used. Always assume the worst case that learner does not recognize any form of variable naming conventions. Additionally, variable naming conventions are very loose. x can represent so many different things across many different fields and sometimes even within the same field.

    Variables should be named consistently. suppose 'v' signified velocity, then as much as possible, 'v' should not represent some other quantity, for example volume. 

    However, this is very difficult as the range and lists of the topic grows, so does the number of variables that need to be described within the archives project. For this reason, scopes must be declared. A particular variable should be defined in a particular scope, say within these group of documents, 'v' should represent velocity, but other than these document, 'v' could represent something else. 




