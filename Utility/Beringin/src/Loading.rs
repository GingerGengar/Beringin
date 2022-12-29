use std::io::Read; //Allow to read file contents
use walkdir; //Allow to go through contents of current directory while skipping hidden files
use std::collections::HashMap; //Allow the creation and usage of hash maps
use crate::Trees; //Read in the structure definitions of single Nodes and all asociated functions

/*Uses the given path to a file to extract file's contents and put it in a string*/
pub fn ExtractFile (RelPath: &std::path::Path, Holder: &mut String){
    //Error message that shows if the file cannot be opened
    let ErrMsg = "Cannot Open File...";
    //Open the file using the given path and set it to the file object
    let mut FileObj = std::fs::File::open(RelPath).expect(ErrMsg);
    //Use the file object to then put all of the file's contents into the given string
    FileObj.read_to_string(Holder);}

/*Function to determine if a file/directory is a dotfile or not*/
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)}

/*Gets toml::value and attempts to parse out the string. returns Option::None if fail*/
fn Toml2String(Parser: &toml::Value, index: &str) -> Option<String> {
    let extracted = Parser.get(index);
    let mut ans = Option::None;
    if extracted != Option::None {
        ans = Some(String::from(extracted.unwrap().as_str().unwrap()));}
    ans}

/*Gets toml::value and Parses out a string vector*/
fn Toml2StringVec(Parser: &toml::Value, index: &str) -> Vec<String> {
    let extracted = &Parser[index];
    let mut ans: Vec<String> = Vec::new();
    let TomlVect = extracted.as_array().unwrap().to_vec();
    for i in TomlVect{
        ans.push(String::from(i.as_str().unwrap()));}
    ans}

/*Gets toml:: value and Parses out OrgHier for a Node*/
fn Toml2OrgHier(Parser: &toml::Value, index: &str) -> Trees::OrgHier {
    let extracted = &Parser[index];
    let Char = extracted.as_str().unwrap();
    let mut ans = Trees::OrgHier::G;
    if Char == "A" {ans = Trees::OrgHier::A;}
    else if Char == "B" {ans = Trees::OrgHier::B;}
    else if Char == "C" {ans = Trees::OrgHier::C;}
    else if Char == "D" {ans = Trees::OrgHier::D;}
    else if Char == "E" {ans = Trees::OrgHier::E;}
    else if Char == "F" {ans = Trees::OrgHier::F;}
    ans}

/*Initialize a new Node of a tree based on data from a string*/
fn ParseNode(DirName: &String, Parser: &toml::Value) -> Trees::Node {
    //Declaring a Node and returning the Node
    Trees::Node {
        Name: DirName.clone(),
        Desc: String::from(Parser["Description"].as_str().unwrap()),
        Pre_Req: Vec::new(),
        Builds_To: Vec::new(),
        Org_Hier: Toml2OrgHier(&Parser, "Organization"),
        Author: Toml2StringVec(&Parser, "Authors"),
        FRationale: Toml2String(&Parser, "Rationale_Name") ,
        FTheory: Toml2String(&Parser, "Theory_Name") ,
        FExamples: Toml2String(&Parser, "Example_Name") ,
        FProblems: Toml2String(&Parser, "Problem_Name") ,
        FSolutions: Toml2String(&Parser, "Solution_Name") ,
    }}

/*Initialize string vector containing dependency of a node based on data from a string*/
fn ParseDep(Parser: &toml::Value) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let extracted = &Parser["Pre_Requisites"];
    let TomlVect = extracted.as_array().unwrap().to_vec();
    //VERY IMPORTANT! WE JUST FOUND A NEW METHOD CALLED TO STRING
    for i in TomlVect{out.push(String::from(i.as_str().unwrap()));}
    //return the vector of strings
    out}

/*Binds Nodes in a pre-requisite built to link*/
fn BindSingle(data: &mut Vec<Trees::Node>, BuiltLoc:usize, PreReqLoc:usize) {
    data[BuiltLoc].Pre_Req.push(PreReqLoc); data[PreReqLoc].Builds_To.push(BuiltLoc);}

/*Iterates through all directories and loads all the metadata Nodes*/
pub fn LoadAllNodes() -> Vec<Trees::Node> {
    //Declaration of vector that holds the entire project's metadata Nodes
    let mut data: Vec<Trees::Node> = Vec::new();
    //Declaration of a vector of vector of strings holding the pre requisite chain
    let mut PreReqRaw: Vec<Vec<String>> = Vec::new();
    //Declaration of a hash map
    let mut Name2Index: HashMap<String, usize> = HashMap::new();
    //Declaration and Initialization of indices
    let mut index: usize = 0;
    //Creation of Directory Iterator. Min Depth and Max Depth set to see just 1 level
    let CurrDir = walkdir::WalkDir::new(".").min_depth(1).max_depth(1).into_iter();
    //Uses the iterator in for loop, but ignores all the hidden files using the is_hidden function
    for i in CurrDir.filter_entry(|e| !is_hidden(e)){
        //Refresh String each loop iteration to contain content of a file
        let mut FContent = String::new();
        //Refresh String each loop iteration to contain name of Node
        let mut NodeName = String::from(i.as_ref()
            .unwrap().path().file_name().unwrap().to_str().unwrap());
        //Append the path with a default metadata file assumed name and read that file
        ExtractFile(&i.unwrap().path().join("Daun.toml"), &mut FContent);
        //Parse the contents of the file using toml library
        let Parser: toml::Value = toml::from_str(&FContent).unwrap();
        //Create a Node based on the directory name and metadata file and append to vector of Nodes
        data.push(ParseNode(&NodeName, &Parser));
        //Append the vector of pre-requisites
        PreReqRaw.push(ParseDep(&Parser));
        //Append to the Hash map and consume the Node Name
        Name2Index.insert(NodeName, index); index += 1;}
    //Check that the length of pre-requisite vector and Node Vector is the same
    assert!(data.len() == PreReqRaw.len());   
    //Bind the various topics together using the hash map and the pre-requisite string vector
    for i in 0..data.len(){//Iterate through all Loaded Nodes
        for j in &PreReqRaw[i]{//Within a single Node, iterate through its pre-req list
            if j != &String::from("Base") {//Don't do anything if a Node's pre-req is "Base"
                //If a Node is not a "Base Node" then perform Binding on its pre-requisites
                BindSingle(&mut data, i, Name2Index[j].clone());}}}
    //Return the vector of the entire project's metadata Nodes 
    data}

/*Function that goes through the vector of Nodes and prints it out*/
pub fn ShowInputData(data: &Vec<Trees::Node>) {
    for i in 0..data.len(){println!("Index: {}\n{}", i, data[i]);}}
