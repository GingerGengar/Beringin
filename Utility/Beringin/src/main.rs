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
    NodeSel.push(0); NodeSel.push(1); NodeSel.push(2); NodeSel.push(3); NodeSel.push(4); NodeSel.push(5); NodeSel.push(6); NodeSel.push(7); NodeSel.push(8); NodeSel.push(9); NodeSel.push(10);
    NodeSel.push(11); NodeSel.push(12); NodeSel.push(13); NodeSel.push(14);

    //Declaration of vector that has the build order
    let mut BuildOrder: Vec<usize> = Vec::new();
    //Produce a Path based on the User Selection
    Assembly::FormPath(&FullMetadata, &mut BuildOrder, & NodeSel);
    println!("{:?}", BuildOrder); //TODO: Delete This
    //Generate the LaTeX Output
    Output::BuildLaTeX(&FullMetadata, &BuildOrder);
}

/*
TODO: We need to desperately add a linear algebra topic
TODO: Currently our File is set to Appending all the time. Is there some way to give it mode of appending sometimes but not appending so our output file does not keep on having to be deleted every time?
TODO: The File optional names have not been implemented, so a single directory of a single Node cannot have its contents be anything other than Theory.tex.
TODO: We need to integrate these topics with UBI somehow and give it some kind of terminal user interface
TODO: Port the rest of the archives
*/
