mod Trees; //Import the structure definitions of a single Node
mod Loading; //Import the File Loading Module to read directories and create Node Vector
mod Assembly; //Import the File containing path formation of the project

/*Loading each Node one at a time as not to overload main memory*/

fn main() {
    //Load all Node Metadata into main memory through a vector
    let FullMetadata: Vec<Trees::Node> = Loading::LoadAllNodes();
    Loading::ShowInputData(&FullMetadata); //TODO: Delete This
    //USER: Figure out where the node we want is located at in the vector
    let NodeSel: usize = 1;
    //Declaration of vector that has the build order
    let mut BuildOrder: Vec<usize> = Vec::new();
    //Produce a Path based on the User Selection
    Assembly::FormPath(&FullMetadata, &mut BuildOrder, & NodeSel);
    println!("{:?}", BuildOrder); //TODO: Delete This

    //Generate the Pre-Amble Part First
    //Generate the contents of the project
    //Generate the end Part of the Document





}
