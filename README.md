# Rationale:

    Document Philosophy:
    Part of the original intention of the Archives Series Project began in an effort to preserve some of the most general and powerful mathematical techniques I learned. The Project began as a way of archiving oneself's skills at the peak of performance in case it would ever be needed again one day. Along the way, I was posed with a few important question "Why is this Archive Necessary?", "Couldn't we just refer back to the original textbooks and sources of the Archives?" "What sets such an Archive different than the multitudes of other textbooks that exist out there?"

    In my humble opinion I believe that writing and giving names to mathematical objects is a pre-requisite to mathematics. It is impossible to do mathematics without definition of terms, and the notation of the terms are closely related to the definition of terms. Since the intimate relationship between notation and definition is so important, uniting various sources with differing notations under a unified consistent notation style ensures that the information discussed in this document is easily readable provided one learns the notation convention defined along the discussion. This guarantees an ease of information absorption and allows the powerful general mathematical theorem in this document to reach its maximum ease of use without compromising generality.

    This calls for the implementation of a series of supposed rules that govern the document. Although not all parts of the documents currently adhere to said rules, this should be a kind of standard that should be achieved in due time for this document to fully implement the principles stated above:

    1.) Theorems presented in the document must contain Proofs

    2.) Each difference in line or after an = sign, there should be a maximum of 2 similar mathematical operations algebraic operations performed.

    3.) Observations should be explained and the types of variables as mathematical objects must be defined within the scope of the theory (Is x a vector? Matrix? Tensor? Real? Complex?) (Is \times a cross product or an algebraic multiplication?)

    4.) The inputs and outputs of the functions produced as a consequence of the mathematical theorems must be clearly defined as said mathematical objects.

    5.) The Document must not include any examples to make it concise and helpful. Examples may be defined in a seperate document. But not this one, because it has to be concise.

    6.) The proof of theorems found in this document should be replaced if a more elegant proof appears.

    Now an interesting bit: Why should I care if this is private or not? I mean it did cost me a lot of effort to produce this but I did write for myself. Distributing it costs me nothing so if it brings advantage, why shouldn't I? This project may serve as the theoretical basis for project River, to be discussed some other time.


# Objectives:

## Resolve knowledge dependency issues

    A particular topic should have its pre-requisite topics made clear. For example, the knowledge of derivatives and integrals is a pre-requisite to differential equations.

    As much as possible, dependency "loops" have to be avoided. This is particularly deadly and here's how it works: Topic A depends on a learner understanding topic B, but topic B depends on the learner understanding topic A. In essence, one has to understand either one of the topics to even be able to understand both and this defeats the purpose of the project. The way such a thing can be avoided is to first ask learner to use suspension of disbelief, or just declare something as a definition, and then the resulting theorem should justify said definition or explain why the suspension of disbelief was initially correct.

    A particular document should always be able to trace its path back to "first principles", which are either nested in physical observation like Newton's 2nd law or some definition agreed upon definition, for example definition of integers.

## Segmentize knowledge into small parts

    Document should be split into small modular pieces. This helps with recyclability. 

    In general documents should be attempted to be kept as small as possible. However, if a group of particular topics are really interrelated, they should be included in the same file. For example, it wouldn't make sense to have integral and differential continuity fluid mechanics split in different documents. Continuity integral form is built on top of differential continuity form, and in many instances, continuity integral refers to many variables of differential continuity. So there is some tradeoff in which knowledge should be grouped together and which could be split. If two topics are not too interrelated, they can be split.

    Doing this also has advantages particularly when collaborating. Files allow the writing of the archives to be more distributed, and makes it easier to do merges.

## Generalization and Abstraction
    
    Generalizations allow equations to become more 'powerful' in that they are able to describe a wider variety of cases.
    
    However, the more generalizations and abstractions are implemented on a particular topic, the harder it is to write, and the harder it is to understand for the inexperienced learner. 

## Enact Variable Naming Standardization
    
    Variables should be named. If a variable was never or defined, it shouldn't be used. Always assume the worst case that learner does not recognize any form of variable naming conventions. Additionally, variable naming conventions are very loose. x can represent so many different things across many different fields and sometimes even within the same field.

    Variables should be named consistently. suppose 'v' signified velocity, then as much as possible, 'v' should not represent some other quantity, for example volume. 

    However, this is very difficult as the range and lists of the topic grows, so does the number of variables that need to be described within the archives project. For this reason, scopes must be declared. A particular variable should be defined in a particular scope, say within these group of documents, 'v' should represent velocity, but other than these document, 'v' could represent something else. 

# To Do:
    
    There are a couple of issues with the reformatting of the project. Particularly in producing a "dependency tree". Here is the vision: A graphical interface exists that shows the dependencies of the topics. Based on the dependencies of the topics, then end user can simply choose a particular topic and a custom-made document can be arranged from the dependency tree to go as far back as first principles as first principles allows. 

    This would mean that each topic under a different circumstance could be shelved under section, subsection, subsubsection, chapter, or even part, which means if we want to make the document structure custom, then the document structure must be dynamically allocated, not static. Therefore, sections, subsubsections, and other similar constructs must be stripped completely from the archive individual content files. 

    So now, how does document generation work? A secondary method, (probably some computer programming language) must have a rather complex data structure that can represent a "tree". Most probably a bunch of objects with pointers to each other. Each node of the tree represents a "content" file in the archive directoy which contains LaTex code about theoretical content. Meaning that each node must have: Name of the topic, and address of the content file. Pointers can then be used to represent what THAT particular topic is a dependency tree to. So if A points to B then A must be the dependent theory to B. So a particular object can point to multiple other objects and the same that multiple other objects can point to a single object. 

    So probably using a programming language write a program which can generate "include" files, which are essentially import statements everywhere and sections and subsection statements, to write the "custom" document.

    The environments must also be discussed. It made sense at some point that each of the archives say math archives and physics archives or fluid archives would have their own separate environment declarations, however this might be an issue. Perhaps it's best to use a tiered inheritance system, wherein say all the archives like math archives and fluid archives share some common environment such as amsmath, but then inside the individual archives, additional environments pertaining to that particular archive may be declared. 

    The way environments are included should be similar to how content is included.

    More experiments needed to be performed on standalone and import and usepackage. This is a major source of headache. For some reason standalone does not allow for begin{center} end{center} environments and I dont know why.

    Of course, more content need to be added, especially structures is very very lacking. Math archives need to be revamped for the PDEs, this is so embarassing.









