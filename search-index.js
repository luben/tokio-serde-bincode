var N=null,E="",T="t",U="u",searchIndex={};
var R=["get_ref","get_mut","into_inner","result","try_from","try_into","borrow_mut","borrow","type_id","typeid","start_send","startsend","poll_complete","ReadBincode","WriteBincode"];

searchIndex["tokio_serde_bincode"]={"doc":"`Stream` and `Sink` adaptors for serializing and…","i":[[3,R[13],"tokio_serde_bincode","Adapts a stream of Bincode encoded buffers to a stream of…",N,N],[3,R[14],E,"Adapts a buffer sink to a value sink by serializing the…",N,N],[11,"new",E,"Creates a new `ReadBincode` with the given buffer stream.",0,[[[T]],["readbincode"]]],[11,R[0],E,"Returns a reference to the underlying stream wrapped by…",0,[[["self"]],[T]]],[11,R[1],E,"Returns a mutable reference to the underlying stream…",0,[[["self"]],[T]]],[11,R[2],E,"Consumes the `ReadBincode`, returning its underlying stream.",0,[[],[T]]],[11,"new",E,"Creates a new `WriteBincode` with the given buffer sink.",1,[[[T]],["writebincode"]]],[11,R[0],E,"Returns a reference to the underlying sink wrapped by…",1,[[["self"]],[T]]],[11,R[1],E,"Returns a mutable reference to the underlying sink wrapped…",1,[[["self"]],[T]]],[11,R[2],E,"Consumes the `WriteBincode`, returning its underlying sink.",1,[[],[T]]],[11,"into",E,E,0,[[],[U]]],[11,"from",E,E,0,[[[T]],[T]]],[11,R[4],E,E,0,[[[U]],[R[3]]]],[11,R[5],E,E,0,[[],[R[3]]]],[11,R[6],E,E,0,[[["self"]],[T]]],[11,R[7],E,E,0,[[["self"]],[T]]],[11,R[8],E,E,0,[[["self"]],[R[9]]]],[11,"into",E,E,1,[[],[U]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[4],E,E,1,[[[U]],[R[3]]]],[11,R[5],E,E,1,[[],[R[3]]]],[11,R[6],E,E,1,[[["self"]],[T]]],[11,R[7],E,E,1,[[["self"]],[T]]],[11,R[8],E,E,1,[[["self"]],[R[9]]]],[11,"poll",E,E,0,[[["self"]],[["poll",["option"]],["option"]]]],[11,"poll",E,E,1,[[["self"]],[["option"],["poll",["option"]]]]],[11,R[10],E,E,0,[[["self"]],[R[11]]]],[11,R[12],E,E,0,[[["self"]],["poll"]]],[11,"close",E,E,0,[[["self"]],["poll"]]],[11,R[10],E,E,1,[[["self"],[U]],[R[11]]]],[11,R[12],E,E,1,[[["self"]],["poll"]]],[11,"close",E,E,1,[[["self"]],["poll"]]]],"p":[[3,R[13]],[3,R[14]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);