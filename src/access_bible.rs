//---例---
//URL:http://localhost:3000/api/v1/verse?book=John&chapter=1&verses=1&version=4016
/*
{"citation":"ヨハネによる福音書 1:1","passage":"初めに言があった。言は神と共にあった。言は神であった。"}
*/
pub struct BibleInfo{
    pub book:String,//書物　例：イザヤ書・マタイによる福音書
    pub chapter:String,//章
    pub verses:String,//節
    pub version:String,//バージョン：口語訳＝4016
}