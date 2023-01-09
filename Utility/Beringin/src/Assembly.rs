use std::collections::HashMap; //Allow the creation and usage of hash maps
use crate::Trees; //Read in the structure definitions of single Nodes and all asociated functions

/*Use recursion to assemble the build order of the nodes based on target*/
fn PathRecurse(TreeRepr: &Vec<Trees::Node>, BuildOrder: &mut Vec<usize>, 
EntryPoint: usize, Map: &mut HashMap<usize, bool>){
    //True only if we have not reached the base of the knowledge tree
    let Enter: bool = TreeRepr[EntryPoint].Pre_Req.len() != 0;
    if Enter {//Enter Recursion only if the conditions above are satisfied
        //Iterate over the pre-requisite of a given node first before doing contents this node
        for i in &TreeRepr[EntryPoint].Pre_Req{
        //Only Enter Recursion if a given pre-requisite has not been added yet
        if Map.get(&i) == Option::None {//Enter Recursion
            PathRecurse(TreeRepr, BuildOrder, i.clone(), Map);}}}
    //Append the Current Node to the hash map
    Map.insert(EntryPoint, true);
    //Append the Current Node to the build Order
    BuildOrder.push(EntryPoint);
}

/*This function is just a setup for the entry point and hash map for the recursion path function*/
pub fn FormPath(FullMetadata: &Vec<Trees::Node>, BuildOrder: &mut Vec<usize>, NodeSel: & Vec<usize>){
    //Declare a Hash Map for a simple search algorithm
    let mut IndexSearch: HashMap<usize, bool> = HashMap::new();
    //Use the location of the node as entry point and iterate over the selected Nodes
    for i in NodeSel{
        if IndexSearch.get(i) == Option::None {
        PathRecurse(FullMetadata, BuildOrder, i.clone(), &mut IndexSearch);}
    }}
