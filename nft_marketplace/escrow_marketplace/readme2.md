
##mark
1、vault_account用来存储托管金额
2、vault_authory是vault_account的权限帐号，同时也是合约的pda帐号
3、escrow_account是用来存储挂单信息的帐号

##todo
1、怎么查询遍历所有的挂单，find program ？
2、兼容多币种
3、还要判断token_account是否可用，没创建的话在合约里面创建，或者用anchor的init_if_need
4、约束必须是K币
5、其他安全隐患
6、seed设计