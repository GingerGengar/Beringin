use std::path::*; //Give Ability to manipulate path addresses
use std::io::Write; //Give Ability to write string to files
use std::fs::*; //Give Ability to Open Files and folders with Certain Options
use walkdir; //Allow to go through all items in a given directory
use crate::Trees;
use crate::Loading;

/*Copy Paste input file to output file*/
fn AppendCopyPaste(InputAddr: &Path, OutputAddr: &Path) {
    //Declare a string that contains the content of the input file
    let mut content: String = String::new();
    //Read in the file to be copied based on given address
    Loading::ExtractFile(InputAddr, &mut content);
    let CheckOutFile = OpenOptions::new().create(true).append(true).open(OutputAddr);
    //Write the file that we want to Copy over
    CheckOutFile.unwrap().write(content.as_bytes()).expect("Write Failed");}

/*Copy Paste ALL entries in a given directory over to another directory (images, programs, etc)*/
fn BatchCopyPaste(InputAddr: &Path, OutputAddr: &Path) {
    //Creation of Directory Iterator. Min Depth and Max Depth set to see just 1 level
    let CurrDir = walkdir::WalkDir::new(InputAddr).min_depth(1).max_depth(1).into_iter();
    //Loop over all of the entrie in the directory for batch copying
    for i in CurrDir.filter_entry(|e| !Loading::is_hidden(e)){
        if i.is_ok() {//Extract the full path of the image we wish to copy
            let IMGPath = i.unwrap().into_path();
            //Extract the file name of the image we wish to copy over
            let CopiedFileName = PathBuf::from((&IMGPath).file_name().unwrap());
            //Form the full destination address for the image
            let CopiedAddress = OutputAddr.clone().join(CopiedFileName);
            //Copy over the image to the destination address
            copy(&IMGPath, CopiedAddress).expect("Batch Copying Failed");}}}

/*Generate Latex Output by Loading each Node one at a time and copy pasting*/
pub fn BuildLaTeX(data: &Vec<Trees::Node>, Order: &Vec<usize>, UserOutDir: &Path) {
    ///////////////////////////////////////////////////////////////////////////////////////
    //This is the creation of the file paths for various directories based on strings
    ///////////////////////////////////////////////////////////////////////////////////////
    //String to contain the root output directory
    let OutDir = UserOutDir.clone().join(PathBuf::from(&"Dahan/"));
    //String to contain the output tex file
    let OutAddr = OutDir.clone().join(PathBuf::from(&"Dahan.tex"));
    //Address of the image directory
    let ImgDirAddr = OutDir.clone().join(PathBuf::from(&"Images/"));
    //Address of the programming directory
    let ProgDirAddr = OutDir.clone().join(PathBuf::from(&"Programs/"));
    ///////////////////////////////////////////////////////////////////////////////////////
    //These are hard-coded assumed about the assumed directory structure
    ///////////////////////////////////////////////////////////////////////////////////////
    //Location of the beginning document  File
    let BegAddr = Path::new("../Environment/Doc_Beginning.tex");
    //Location of the Ending document File
    let EndAddr = Path::new("../Environment/Doc_Ending.tex");
    //Create a new directory at the outer address
    let dir = create_dir(OutDir);
    ///////////////////////////////////////////////////////////////////////////////////////
    //Section that handles the main copy-pasting function calls to build the final output
    ///////////////////////////////////////////////////////////////////////////////////////
    if (dir.is_ok()){//Only start building the output files and directories if all is okay
        //Create Image Directory
        let ImgDir = create_dir(&ImgDirAddr);
        //Create Programming Directory
        let ProgDir = create_dir(&ProgDirAddr);
        //Generate the Pre-Amble Part First
        AppendCopyPaste(&BegAddr, &OutAddr);
        //Iterate through the build order which contains the ordered Nodes ready to be copy pasted
        for i in Order{//Perform the Copy Pasting for each element in the build order vector
            //Use Node Name to determine its relative path and the default theory file name
            AppendCopyPaste(&Path::new(&(data[*i].Name.clone()+&"/Theory.tex")), &OutAddr);
            //Use Node Name to copy all images over
            BatchCopyPaste(&Path::new(&(data[*i].Name.clone()+&"/Images/")), &ImgDirAddr);
            //Use Node Name to copy all of the programs over
            BatchCopyPaste(&Path::new(&(data[*i].Name.clone()+&"/Programs/")), &ProgDirAddr);}
        //Generate the end Part of the Document
        AppendCopyPaste(&EndAddr, &OutAddr);}
    else {//If creating a new directory fails, show why
        println!("{}", dir.err().unwrap());}}

