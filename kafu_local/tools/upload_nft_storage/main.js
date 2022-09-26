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
    console.log("0001")
    var image_cid = null;
    let image_files = fs.readdirSync(path.join(project_dir,"image"));
    for (let id = start; id < end; id++) {
        let json_name = id+".json";
        let metadata_path = path.join(project_dir,"json",json_name);
        //房间的都一样，目前都是一个每个类型的图片可以做成多个nft的,所以image只上传一次而且是0.png而不是对应的{id}.png
        console.log("----- ",image_files[0])
        let image_path = path.join(project_dir,"image",image_files[0]);
        console.log("--2--- ",image_path)

        if (!image_cid ){
            let image_data =  fs.readFileSync(image_path);
            image_cid = await storage.storeBlob(new Blob([image_data]))
            console.log("%s cid %s",image_path,image_cid);
        }

        let metadata = {
            name: name + " #" + id,
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
    }
    const metadata_dir_cid = await storage.storeDirectory(files_data)
    console.log("name %s dir cid: https://%s.ipfs.nftstorage.link",name,metadata_dir_cid);
}

async function main() {
    const storage = new NFTStorage({ endpoint, token })
    //await upload(storage,"./1.png","room_0926_0001",0);
    //todo: config
    //room
    for (let level = 1; level < 6; level++) {
        await upload_dir(storage, "./resource/room/level"+level, "LEVEL"+level+"-ROOM", 0, 1000);
    }

    //avatar_frame
    for (let i = 0; i < 15; i++) {
        await upload_dir(storage, "./resource/avatar_frame/"+i, "frame"+i, 0, 1000);
    }

    //vehicle
    for (let i = 0; i < 15; i++) {
        await upload_dir(storage, "./resource/vehicle/"+i, "vehicle"+i, 0, 1000);
    }

}
main()