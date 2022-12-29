use std::path::Path; //Give Ability to create a path address
use std::io::Write; //Give Ability to write string to files
use std::fs::OpenOptions; //Give Ability to Open File with Certain Options
use crate::Trees;
use crate::Loading;

/*Copy Paste input file to output file*/
fn AppendCopyPaste(InputAddr: &Path, OutputAddr: &Path) {
    //Declare a string that contains the content of the input file
    let mut content: String = String::new();
    //Read in the file to be copied based on given address
    Loading::ExtractFile(InputAddr, &mut content);
    //Open the File we want to Write To
    let mut OutFile = OpenOptions::new().append(true).create(true)
        .open(OutputAddr).expect("Opening File Failed");
    //Write out the file that we want to Copy
    OutFile.write(content.as_bytes()).expect("Write Failed");}

/*Generate Latex Output by Loading each Node one at a time and copy pasting*/
pub fn BuildLaTeX(data: &Vec<Trees::Node>, Order: &Vec<usize>) {
    //Location of where the intended output file would be
    let OutAddr = Path::new("/home/hyahoos/Utility/Free/Testing.tex");
    //Location of the beginning document  File
    let BegAddr = Path::new("../Environment/Doc_Beginning.tex");
    //Location of the Ending document File
    let EndAddr = Path::new("../Environment/Doc_Ending.tex");
    //Generate the Pre-Amble Part First
    AppendCopyPaste(&BegAddr, &OutAddr);
    //Iterate through the build order which contains the ordered Nodes ready to be copy pasted
    for i in Order{//Perform the Copy Pasting for each element in the build order vector
        //Use the Name of the Node to figure out its relative path and the default theory file name
        AppendCopyPaste(&Path::new(&(data[*i].Name.clone()+&"/Theory.tex")), &OutAddr);}
    //Generate the end Part of the Document
    AppendCopyPaste(&EndAddr, &OutAddr);}

