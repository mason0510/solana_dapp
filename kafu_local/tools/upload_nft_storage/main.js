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

//房间的直接从start开始铸造新的一批
//非房间的需要png命名从上一批的last-id续上
async function upload_dir(storage,project_dir,name,start,end){
    var files_data = []
    var id = 0;
    console.log("0001")
    var image_cid = null;
    let image_files = fs.readdirSync(path.join(project_dir,"image"));
    for (let i = start; i < end; i++) {
        let json_name = id+".json";
        let metadata_path = path.join(project_dir,"json",json_name);
        //房间的都一样
        if (name.includes("ROOM") && image_cid ){
           console.log("image is already upload")
        }else {
            let image_path = path.join(project_dir,"image",image_files[i]);
            let image_data =  fs.readFileSync(image_path);
            image_cid = await storage.storeBlob(new Blob([image_data]))
            console.log("%s cid %s",image_path,image_cid);
        }

        let metadata = {
            name: name,
            symbol: '',
            description: '',
            image: 'https://'+image_cid+'.ipfs.nftstorage.link',
            external_url:"",
            attributes: [{trait_type: "id",value: id}]
        };
        fs.writeFileSync(metadata_path,JSON.stringify(metadata));
        let file_data = new File([  fs.readFileSync(metadata_path)],json_name);
        console.log("--",metadata_path,JSON.stringify(metadata));
        files_data.push(file_data);
        id++;
    }
    const metadata_dir_cid = await storage.storeDirectory(files_data)
    console.log("dir cid: https://%s.ipfs.nftstorage.link",metadata_dir_cid);
}

async function main() {
    const storage = new NFTStorage({ endpoint, token })
    //await upload(storage,"./1.png","room_0926_0001",0);
    //todo: config
    await upload_dir(storage,"./resource/room1","LEVEL1-ROOM",0,100);
}
main()