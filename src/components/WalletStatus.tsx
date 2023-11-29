"use client"
import { Principal } from "@dfinity/principal"
import { ShadowInnerIcon } from "@radix-ui/react-icons"
import { objectToString } from "lib/utils"
import { useState } from "react"
import { useSystemMethod } from "service/system"
import { Button } from "./ui/button"
import { Input } from "./ui/input"

interface WalletStatusProps {
  canisterId?: string
}

const WalletStatus: React.FC<WalletStatusProps> = ({ canisterId }) => {
  const { call, data, error, loading } = useSystemMethod(
    "get_user_canister_status"
  )

  const [input, setInput] = useState(canisterId || "")

  const installWalletHandler = async () => {
    if (!input) return

    const principal = Principal.fromText(input)

    const res = await call(principal)
    console.log(res)
  }

  return (
    <div>
      <div className="flex items-center">
        <Input
          icon={<ShadowInnerIcon className="ml-[4px]" />}
          value={input}
          onChange={e => setInput(e.target.value)}
          placeholder="Canister ID"
          round="left"
          iconSize="sm"
        />
        <Button
          round="right"
          variant="outline"
          color="secondary"
          onClick={installWalletHandler}
        >
          Wallet Status
        </Button>
      </div>
      <label>Response: &nbsp;</label>
      {loading ? <span>Loading...</span> : null}
      {error ? <span>Error: {JSON.stringify(error)}</span> : null}
      {data && <span>{objectToString(data)}</span>}
      {/* {hash ? (
          <span>Hash: {hash}</span>
        ) : (
          <InstallWallet canisterId={"ajuq4-ruaaa-aaaaa-qaaga-cai"} />
        )} */}
    </div>
  )
}

export default WalletStatus
