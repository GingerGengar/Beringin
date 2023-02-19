use std::path::*;
mod Trees; //Import the structure definitions of a single Node
mod Loading; //Import the File Loading Module to read directories and create Node Vector
mod Assembly; //Import the File containing path formation of the project
mod Output; //Import the File containing output latex file formation of project

fn main() {
    //Load all Node Metadata into main memory through a vector
    let FullMetadata: Vec<Trees::Node> = Loading::LoadAllNodes();
    Loading::ShowInputData(&FullMetadata); //TODO: Delete This
    //USER: Figure out where the node we want is located at in the vector
    let mut NodeSel: Vec<usize> = Vec::new();
    NodeSel.push(7);
    //Declaration of vector that has the build order
    let mut BuildOrder: Vec<usize> = Vec::new();
    //Produce a Path based on the User Selection
    Assembly::FormPath(&FullMetadata, &mut BuildOrder, & NodeSel);
    println!("{:?}", BuildOrder); //TODO: Delete This
    //Where the user wants to put the results
    let UserOutDir = Path::new("/home/hyahoos/Utility/Free");
    //Generate the LaTeX Output
    Output::BuildLaTeX(&FullMetadata, &BuildOrder, &UserOutDir);
}

/*
TODO: Give some ability to check the integrity of the packages, noting which pre-requisite names do not match
TODO: Give some utilities like renaming a Node such that not only the directory name has changed but all Daun.toml files change their names accordingly.
TODO: We need to desperately add a linear algebra topic
TODO: The File optional names have not been implemented, so a single directory of a single Node cannot have its contents be anything other than Theory.tex.
TODO: We need to integrate these topics with UBI somehow and give it some kind of terminal user interface
TODO: Port the rest of the archives
*/
