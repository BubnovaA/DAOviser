// voteBot.js
import { ethers } from 'ethers';
import { createSafeClient } from '@safe-global/sdk-starter-kit';
import dotenv from 'dotenv';
dotenv.config();
import axios from 'axios';
import cron from 'node-cron';


async function voteProposal() {
  try {
    const response = await axios.get('http://localhost:8080/get_prop_and_rec');
    const data = response.data;

    // Expected response format: { proposalId: number, voteOption: number }
    const { proposalId, voteOption } = data;
    if (proposalId === undefined || voteOption === undefined) {
      console.log('Invalid data received:', data);
      return;
    }
    console.log(`Received voting data - Proposal ID: ${proposalId}, Vote Option: ${voteOption}`);

    // Load environment variables
    const { RPC_URL, PRIVATE_KEY, SAFE_ADDRESS, TALLY_ADDRESS } = process.env;
    if (!RPC_URL || !PRIVATE_KEY || !SAFE_ADDRESS || !TALLY_ADDRESS) {
      throw new Error('Required environment variables are not set');
    }

    // Validate addresses using ethers.isAddress
    if (!ethers.isAddress(SAFE_ADDRESS)) {
      throw new Error('Invalid SAFE_ADDRESS');
    }
    if (!ethers.isAddress(TALLY_ADDRESS)) {
      throw new Error('Invalid TALLY_ADDRESS');
    }

    // Create ethers provider and signer (using ethers v6)
    const provider = new ethers.JsonRpcProvider(RPC_URL);
   
    const safeClient = await createSafeClient({
		provider: RPC_URL,
		signer: PRIVATE_KEY,
		safeAddress: ethers.getAddress(SAFE_ADDRESS),
	});

    console.log(safeClient);
    // Define the Tally contract ABI (assumes function castVote(uint256 proposalId, uint8 vote))
    const TALLY_ABI = [
      'function castVote(uint256 proposalId, uint8 vote) external'
    ];

    // Create a contract instance for Tally (read-only; used only for encoding)
    const tallyContract = new ethers.Contract(TALLY_ADDRESS, TALLY_ABI, provider);

    // Encode the function call data for voting
    const voteData = tallyContract.interface.encodeFunctionData('castVote', [
      proposalId,
      voteOption
    ]);

    // Prepare the transaction object
    const transaction = {
      to: TALLY_ADDRESS,
      value: '0',
      data: voteData,
    };

    console.log('Prepared vote transaction:', transaction);
    console.log('Sending transaction via Gnosis Safe...');

    // Send transaction via Safe Wallet
    const txResult = await safeClient.send({
      transactions: [transaction],
    });

    const safeTxHash = txResult.transactions.safeTxHash;
    console.log('Vote transaction sent. SafeTxHash:', safeTxHash);
  } catch (error) {
    console.error('Error while voting:', error);
  }
}

// Schedule the voteProposal function to run every hour at minute 0
cron.schedule('0 * * * *', () => {
    console.log(`\nStarting voting task: ${new Date().toISOString()}`);
    voteProposal();
  });
  

