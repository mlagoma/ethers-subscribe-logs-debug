import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.9",
  defaultNetwork: 'development',
  networks: {
    development: {
      url: 'http://127.0.0.1:8888'
    }
  }
};

export default config;
