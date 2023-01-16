use std::fmt;

/*7 levels of LaTeX organizational hierarchy*/
pub enum OrgHier {
    //A is largest, like part and G is the lowest, like sub-paragraph
    A, B, C, D, E, F, G,
}

/*This contains just the metadata of a single knowledge archive Node*/
pub struct Node {//Core Metadata of a single Node
    pub Name: String, //Name of the Node
    pub Desc: String, //Short Description of what the Node contains
    pub Pre_Req: Vec<usize>, //Topic Names of this Node's Pre-Requisites
    pub Builds_To: Vec<usize>, //Topic Names of what this Node builds into
    pub Org_Hier: OrgHier, //Where in the Latex organizational level does this reside in
    pub Author: Vec<String>, //Who are the contributing authors to this node
    //Optional Specificity on the Node's file Names
    pub FRationale: Option<String>, //Rationale File-Name (optional)
    pub FTheory: Option<String>, //Theory File-Name (optional)
    pub FExamples: Option<String>, //Examples File-Name (optional)
    pub FProblems: Option<String>, //Problems File-Name (optional)
    pub FSolutions: Option<String>, //Solutions File-Name (optional)
}

/*Given an OrgHier object, output the corresponding Name in String*/
fn Org2Str(OrgObj: &OrgHier) -> String {
    match OrgObj { //Uses the match control flow to convert representations
        OrgHier::A => String::from("A"),
        OrgHier::B => String::from("B"), 
        OrgHier::C => String::from("C"), 
        OrgHier::D => String::from("D"), 
        OrgHier::E => String::from("E"), 
        OrgHier::F => String::from("F"), 
        OrgHier::G => String::from("G"), }}

//Implement std::fmt::Display on Node so we can see what is inside Node
impl fmt::Display for Node {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let mut Out: String = String::new();
        //Just Appending the Strings of the Name and Description to the output String
        Out = Out + "Node Name: " + &self.Name.to_string() + "\n";
        Out = Out + "Desc: " + &self.Desc.to_string() + "\n";
        //Using the DEBUG formatter to print the vectors of usizes for both pre-req and build to
        Out = Out + "Pre_Req: " + &format!("{:?}", &self.Pre_Req).to_string()  + "\n";
        Out = Out + "Builds_To: " + &format!("{:?}", &self.Builds_To).to_string()  + "\n";
        //Uses some function Org2Str to convert the enum to its corresponding string representation
        Out = Out + "Org_Hier: " + &Org2Str(&self.Org_Hier) + "\n";
        //Using the DEBUG formatter to print the list of authors
        Out = Out + "Author: " + &format!("{:?}", &self.Author).to_string()  + "\n";
        //Return the result of a display using write!
        write!(f, "{}", Out)}}

