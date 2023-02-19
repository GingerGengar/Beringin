use std::path::Path; //Give Ability to create a path address
use std::io::Write; //Give Ability to write string to files
use std::fs::OpenOptions; //Give Ability to Open File with Certain Options
use crate::Trees;
use crate::Loading;

pub struct Status {//These are to tell if there are any errors in the file related processes
    pub FirstFilePass: bool, //True for first copy pasting run, false otherwise
    pub SomeError: bool, //This is to tell if some error has occured and to abort all operations
}

/*Copy Paste input file to output file*/
fn AppendCopyPaste(InputAddr: &Path, OutputAddr: &Path, health: &mut Status) {
    if !(health.SomeError) {//Only continue if we are clear of errors and all is good
        //Declare a string that contains the content of the input file
        let mut content: String = String::new();
        //Read in the file to be copied based on given address
        Loading::ExtractFile(InputAddr, &mut content);
        //Check if this is the first time we are opening the contents of the file
        if (health.FirstFilePass) {
            //Open the file in create new mode, return error if it already exist
            let CheckOutFile = OpenOptions::new().write(true).create_new(true).open(OutputAddr);
            if (CheckOutFile.is_ok()){//Make new file if all is okay
                //Write out the file that we want to Copy
                CheckOutFile.unwrap().write(content.as_bytes()).expect("Write Failed");} 
            else {//If error exist, show reason and immediately stop trying to write to files
                println!("{}", CheckOutFile.err().unwrap()); health.SomeError = true;}
            //Regardless of what happens, we have done our first file pass check, so...
            health.FirstFilePass = false;}
        else {//Open the file in appending mode if we have already already created the file
            let CheckOutFile = OpenOptions::new().create(true).append(true).open(OutputAddr);
            //Write the file that we want to Copy over
            CheckOutFile.unwrap().write(content.as_bytes()).expect("Write Failed");}}}

/*Generate Latex Output by Loading each Node one at a time and copy pasting*/
pub fn BuildLaTeX(data: &Vec<Trees::Node>, Order: &Vec<usize>) {
    //Location of where the intended output file would be
    let OutAddr = Path::new("/home/hyahoos/Utility/Free/Testing.tex");
    //Location of the beginning document  File
    let BegAddr = Path::new("../Environment/Doc_Beginning.tex");
    //Location of the Ending document File
    let EndAddr = Path::new("../Environment/Doc_Ending.tex");
    //Create a new status object
    let mut health = Status{FirstFilePass:true, SomeError:false};
    //Generate the Pre-Amble Part First
    AppendCopyPaste(&BegAddr, &OutAddr, &mut health);
    //Iterate through the build order which contains the ordered Nodes ready to be copy pasted
    for i in Order{//Perform the Copy Pasting for each element in the build order vector
        //Use the Name of the Node to figure out its relative path and the default theory file name
        AppendCopyPaste(&Path::new(&(data[*i].Name.clone()+&"/Theory.tex")), 
        &OutAddr, &mut health);}
    //Generate the end Part of the Document
    AppendCopyPaste(&EndAddr, &OutAddr, &mut health);}

