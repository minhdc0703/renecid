import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RenecId } from "../target/types/renec_id";

describe("renecId", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RenecId as Program<RenecId>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
