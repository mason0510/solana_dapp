import  fs from 'fs'
import { NFTStorage, File } from 'nft.storage'
import path from 'path'

const endpoint = 'https://api.nft.storage' // the default
const token = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJkaWQ6ZXRocjoweDBmQzc0MTY4RjUxZTk0NjU3ODE4N0EwMTBmMjdBMjllOTFmQzZjYmEiLCJpc3MiOiJuZnQtc3RvcmFnZSIsImlhdCI6MTY1OTYwMzkyNzI3MywibmFtZSI6InNvbGFuYSJ9.cNyVRbXYsObBPVvXDax2kYwMRtROHEFim9hVdfI-0gg' // your API key from https://nft.storage/manage

async function upload_kin(storage,project_dir,name){
    console.log("0001")
    let metadata_path = path.join(project_dir,"json","kin.json");
    let image_data = fs.readFileSync(path.join(project_dir,"image/0.png"));
    let image_cid = await storage.storeBlob(new Blob([image_data]))
    console.log("collection image %s cid %s",name,image_cid);
    let metadata = {
        name: name,
        symbol: '',
        description: '',
        image: 'https://'+image_cid+'.ipfs.nftstorage.link',
        external_url:"",
        attributes: []
    };
    fs.writeFileSync(metadata_path,JSON.stringify(metadata));
    let file_data = new File([  fs.readFileSync(metadata_path)],"kin.json");
    const metadata_dir_cid = await storage.storeDirectory([file_data])
    return "https://"+metadata_dir_cid+".ipfs.nftstorage.link/kin.json"
}

async function upload_collection(storage,project_dir,name){
    console.log("0001")
    let metadata_path = path.join(project_dir,"json","collection.json");
    let image_data = fs.readFileSync(path.join(project_dir,"image/0.png"));
    let image_cid = await storage.storeBlob(new Blob([image_data]))
    console.log("collection image %s cid %s",name,image_cid);
    let metadata = {
        name: name,
        symbol: '',
        description: '',
        image: 'https://'+image_cid+'.ipfs.nftstorage.link',
        external_url:"",
        attributes: []
    };
    fs.writeFileSync(metadata_path,JSON.stringify(metadata));
    let file_data = new File([  fs.readFileSync(metadata_path)],"collection.json");
    const metadata_dir_cid = await storage.storeDirectory([file_data])
    return "https://"+metadata_dir_cid+".ipfs.nftstorage.link/collection.json"
}

//房间的直接从start开始铸造新的一批
//非房间的需要png命名从上一批的last-id续上
async function upload_dir(storage,project_dir,name,decoration_id,decoration_url,start,end){
    var files_data = []
    var image_cid = null;
    let image_files = fs.readdirSync(path.join(project_dir,"image"));
    for (let id = start; id < end; id++) {
        let json_name = id+".json";
        let metadata_path = path.join(project_dir,"json",json_name);
        //房间的都一样，目前都是一个每个类型的图片可以做成多个nft的,所以image只上传一次而且是0.png而不是对应的{id}.png
        let image_path = path.join(project_dir,"image",image_files[0]);

        if (!image_cid ){
            let image_data =  fs.readFileSync(image_path);
            image_cid = await storage.storeBlob(new Blob([image_data]))
            //console.log("%s cid %s",image_path,image_cid);
        }

        let metadata = {
            name: name + " #" + id,
            symbol: '',
            description: '',
            image: 'https://'+image_cid+'.ipfs.nftstorage.link',
            external_url:"",
            decoration_id:decoration_id,
            decoration_url:decoration_url,
            attributes: [{trait_type: "id",value: id}]
        };
        fs.writeFileSync(metadata_path,JSON.stringify(metadata));
        let file_data = new File([  fs.readFileSync(metadata_path)],json_name);
        files_data.push(file_data);
    }
    const metadata_dir_cid = await storage.storeDirectory(files_data)
    //console.log("name %s dir cid: https://%s.ipfs.nftstorage.link",name,metadata_dir_cid);
    return "https://"+metadata_dir_cid+".ipfs.nftstorage.link"
}

async function main() {
    console.log("000__001")
    const storage = new NFTStorage({ endpoint, token })
    var upload_cids = []
    //Kcoin
    let kcoin_uri = await upload_kin(storage,"./resource/kin/","Kin")
    console.log("Kin_uri ",kcoin_uri);
    //todo: config
    //room
    for (let level = 1; level < 2; level++) {
        let name = "LEVEL"+level+"-ROOM";
        let collection_uri = await upload_collection(storage,"./resource/room/level"+level,name + " collection")
        let token_uri = await upload_dir(storage, "./resource/room/level"+level, name, 0,"",0, 100);
        upload_cids.push({project:name,collection_uri:collection_uri,token_uri:token_uri,supply:100})
        console.log("complete upload %s",name)
    }

    //avatar_frame

    //todo： from config file
    let frame_infos = [
        "frame1@408122282997715373@https://dev-cos.kafumena.com/decoration/lottie/package/PK_220517200926_1_app.zip",
        "frame2@427508044499262743@https://dev-cos.kafumena.com/decoration/lottie/package/dianfeng_220928135014_1_app.zip",
        "frame3@427508535551597847@https://dev-cos.kafumena.com/decoration/lottie/package/maikefeng_220928135458_2_app.zip",
        "frame4@363457616912849722@https://devp-test.oss-cn-hongkong.aliyuncs.com/decoration/lottie/package/universe.zip",
        "frame5@427509088545414423@https://dev-cos.kafumena.com/decoration/lottie/package/limao_220928140022_3_app.zip",
        "frame6@398353651321543806@https://ali-oss-dev.ttchat.com/decoration/lottie/package/rose_queen.zip",
        "frame7@427509529735863575@https://dev-cos.kafumena.com/decoration/lottie/package/dashenblue_220928140500_5_app.zip",
        "frame8@427509815871282455@https://dev-cos.kafumena.com/decoration/lottie/package/dashenred_220928140448_4_app.zip",
        "frame9@398354362222517374@https://ali-oss-dev.ttchat.com/decoration/lottie/package/supercar.zip",
        "frame10@427509987317652759@https://dev-cos.kafumena.com/decoration/lottie/package/mojinjiaowei_220928140911_6_app.zip",
        "frame11@427510053084339479@https://dev-cos.kafumena.com/decoration/lottie/package/youlongcuanshuo_220928140924_7_app.zip",
        "frame12@400263070837905261@https://ali-oss-dev.ttchat.com/decoration/lottie/package/blue_wings.zip",
        "frame13@400262741568264045@https://ali-oss-dev.ttchat.com/decoration/lottie/package/mech_warrior.zip",
        "frame14@427510211645807895@https://dev-cos.kafumena.com/decoration/lottie/package/rosebule_220928141142_8_app.zip",
        "frame15@427510265852992791@https://dev-cos.kafumena.com/decoration/lottie/package/rosered_220928141150_9_app.zip",
    ]
    for (let i = 0; i < 15; i++) {
        let [name,decoration_id,decoration_url] = frame_infos[i].split('@');
        let collection_uri = await upload_collection(storage,"./resource/avatar_frame/"+i,name + " collection")
        let token_uri = await upload_dir(storage, "./resource/avatar_frame/"+i, name, parseInt(decoration_id),decoration_url,0, 100);
        upload_cids.push({project:name,collection_uri:collection_uri,token_uri:token_uri,supply:100})
        console.log("complete upload %s",name)
    }

    //vehicle
    let vehicle_infos = [
        "vehicle1@408268155085919895@https://kafu-dev-1311489648.cos.eu-frankfurt.myqcloud.com/decoration/lottie/package/enter_room_eagle_220525170009_4_app.zip",
        "vehicle2@400269554225256301@https://ali-oss-dev.ttchat.com/decoration/lottie/package/enter_room_blackcar.zip",
        "vehicle3@400269608046565229@https://ali-oss-dev.ttchat.com/decoration/lottie/package/enter_room_camel.zip",
        "vehicle4@414486269855273232@https://dev-cos.kafumena.com/decoration/lottie/package/enter_room_cruiseship_220705142630_2_app.zip",
        "vehicle5@400269656801154925@https://ali-oss-dev.ttchat.com/decoration/lottie/package/enter_room_hotairballoon.zip",
        "vehicle6@400269688677865325@https://dev-cos.kafumena.com/decoration/lottie/package/enter_room_Red_motorcycle_220705204453_1_app.zip",
        "vehicle7@409261211234866259@https://kafu-dev-1311489648.cos.eu-frankfurt.myqcloud.com/decoration/lottie/package/enter_room_wolf_220525170103_5_app.zip",
        "vehicle8@427511872455974167@https://dev-cos.kafumena.com/decoration/lottie/package/enter_room_helicopter_220519141402_1_app.zip",
        "vehicle9@407106325881821498@https://dev-cos.kafumena.com/decoration/lottie/package/enter_room_unique1_220518211800_1_app.zip",
        "vehicle10@408268068096054935@https://dev-cos.kafumena.com/decoration/lottie/package/enter_room_carriage1_220518211509_1_app.zip",
        "vehicle11@408267880795215511@https://dev-cos.kafumena.com/decoration/lottie/package/enter_room_lion_220518210205_1_app.zip",
        "vehicle12@399392678837489895@https://ali-oss-dev.ttchat.com/decoration/lottie/package/enter_room_lv5new7.zip",
        "vehicle13@398824307276387591@https://ali-oss-dev.ttchat.com/decoration/lottie/package/enter_room_lv4new3.zip",
        "vehicle14@398824200422298887@https://ali-oss-dev.ttchat.com/decoration/lottie/package/enter_room_lv3new3.zip",
        "vehicle15@427512285628472599@https://dev-cos.kafumena.com/decoration/lottie/package/enter_room_horse11_220928143022_1_app.zip",
    ];
    for (let i = 0; i < 15; i++) {
        let [name,decoration_id,decoration_url] = vehicle_infos[i].split('@');
        let collection_uri = await upload_collection(storage,"./resource/vehicle/"+i,name + " collection")
        let token_uri = await upload_dir(storage, "./resource/vehicle/"+i, name , parseInt(decoration_id),decoration_url,0, 100);
        upload_cids.push({project:name,collection_uri:collection_uri,token_uri:token_uri,supply:100})
        console.log("complete upload %s",name)
    }
    await fs.promises.writeFile("./resource/upload_cids.json",JSON.stringify(upload_cids,null,"\t"));

}
main()