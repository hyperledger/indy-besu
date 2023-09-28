import { web3 } from '../environment';
import { Account as Web3Account } from 'web3-core';

export class Account {
    public address: string
    public privateKey: string
    public signer: Signer

    constructor(data?: AccountInfo) {
        const provider = new ethers.JsonRpcProvider(host)
        if (data) {
            this.signer = new ethers.Wallet(data.privateKey, provider)
            this.address = data.address
            this.privateKey = data.privateKey
        } else {
            const account = web3.eth.accounts.create()
            this.signer = new ethers.Wallet(account.privateKey, provider)
            this.address = account.address
            this.privateKey = account.privateKey
        }
    }

    public get did() {
        return `did:${environment.did.method}:${environment.network.name}:${this.methodSpeceficId}`
    }

    public get methodSpeceficId() {
        const parts = [
            this.address.substring(2, 10),
            this.address.substring(10, 14),
            this.address.substring(14, 18),
            this.address.substring(18, 22),
            this.address.substring(22, 34),
        ]

        return parts.join('-')
    }

    public get didDocument() {
        return createBaseDidDocument(this.did)
    }
}
