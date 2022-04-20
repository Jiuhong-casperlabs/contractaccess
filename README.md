=============deploy other contract===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 50000000000 \
--session-path /home/jh/mywork/contractaccess/contract/target/wasm32-unknown-unknown/release/other.wasm \
--secret-key /home/jh/keys/test99/secret_key.pem 

=> 
other_contract
hash-d714705e4232612bc234b3f283b88815208c3155f143fbd08e6896f0cad0d67f

=============deploy main contract===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 40000000000 \
--session-arg "other_contract:String='contract-d714705e4232612bc234b3f283b88815208c3155f143fbd08e6896f0cad0d67f'" \
--session-path /home/jh/mywork/contractaccess/contract/target/wasm32-unknown-unknown/release/contract.wasm \
--secret-key /home/jh/keys/test99/secret_key.pem 

=>my_contract
hash-12e2c0f4d157b00dd9faef54819bb9d0713d0e2f86f1f62a6ed1a8199396fc3f

=============call_access_other from other_contract===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 5000000000 \
--session-hash hash-d714705e4232612bc234b3f283b88815208c3155f143fbd08e6896f0cad0d67f \
--session-entry-point "call_access_other" \
--session-arg "main_contract:String='contract-12e2c0f4d157b00dd9faef54819bb9d0713d0e2f86f1f62a6ed1a8199396fc3f'" \
--secret-key /home/jh/keys/test1/secret_key.pem 


=============call test2 from other account directly===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 5000000000 \
--session-hash hash-12e2c0f4d157b00dd9faef54819bb9d0713d0e2f86f1f62a6ed1a8199396fc3f \
--session-entry-point "test2" \
--secret-key /home/jh/keys/test1/secret_key.pem 
=>
"error_message": "Invalid context"
=============deploy another_contract===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 40000000000 \
--session-path /home/jh/mywork/contractaccess/contract/target/wasm32-unknown-unknown/release/another.wasm \
--secret-key /home/jh/keys/test99/secret_key.pem 

=> 
another_contract
hash-07b0fdc2d76183c590816f1374a6a91812e6c9828856b74066502219deb27171

=============call_access_other from another_contract===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 5000000000 \
--session-hash hash-07b0fdc2d76183c590816f1374a6a91812e6c9828856b74066502219deb27171 \
--session-entry-point "call_access_other" \
--session-arg "main_contract:String='contract-12e2c0f4d157b00dd9faef54819bb9d0713d0e2f86f1f62a6ed1a8199396fc3f'" \
--secret-key /home/jh/keys/test1/secret_key.pem 

=>
"error_message": "Invalid context"