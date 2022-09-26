import  fs from 'fs'
import { NFTStorage, File } from 'nft.storage'
import path from 'path'

const endpoint = 'https://api.nft.storage' // the default
const token = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJkaWQ6ZXRocjoweDBmQzc0MTY4RjUxZTk0NjU3ODE4N0EwMTBmMjdBMjllOTFmQzZjYmEiLCJpc3MiOiJuZnQtc3RvcmFnZSIsImlhdCI6MTY1OTYwMzkyNzI3MywibmFtZSI6InNvbGFuYSJ9.cNyVRbXYsObBPVvXDax2kYwMRtROHEFim9hVdfI-0gg' // your API key from https://nft.storage/manage
async function upload(storage,image_path,name,id){
    const metadata = await storage.storage({
        name: name,
        description: '',
        image: new File([await fs.promises.readFile(image_path)], id + '.png', {
            type: 'image/png',
        }),
        external_url:"",
        attributes: [{trait_type: "id",value: id}]
    }).rename(id+'.json')
    console.log('IPFS URL for the metadata:', metadata.url)
}

async function upload_dir(storage,project_dir,name){
    var files_data = []
    var id = 0;
    console.log("0001")
    let image_files = fs.readdirSync(path.join(project_dir,"image"));
    for (let i = 0; i < image_files.length; i++) {
        let json_name = id+".json";
        let metadata_path = path.join(project_dir,"json",json_name);
        console.log("---%s",metadata_path);
        let image_path = path.join(project_dir,"image",image_files[i]);
        console.log("---%s",image_path);
        let image_data =  fs.readFileSync(image_path);
        const cid = await storage.storeBlob(new Blob([image_data]))
        console.log("%s cid %s",image_path,cid);
        let metadata = {
            name: name,
            symbol: '',
            description: '',
            image: 'https://'+cid+'.ipfs.nftstorage.link',
            external_url:"",
            attributes: [{trait_type: "id",value: id}]
        };
        fs.writeFileSync(metadata_path,JSON.stringify(metadata));
        let file_data = new File([ metadata_path],json_name);
        files_data.push(file_data);
        id++;
    }
    const metadata_dir_cid = await storage.storeDirectory(files_data)
    console.log("dir cid: https://%s.ipfs.nftstorage.link",metadata_dir_cid);
}

async function main() {
    const storage = new NFTStorage({ endpoint, token })
    //await upload(storage,"./1.png","room_0926_0001",0);
    await upload_dir(storage,"./resource/car","room_0926_0001");
}
main()