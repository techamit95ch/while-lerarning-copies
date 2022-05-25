import { AiFillPlayCircle } from "react-icons/ai";
import { SiEthereum } from "react-icons/si";
import { BsInfoCircle } from "react-icons/bs";
import { Loader } from "./";
const Welcome = () => {
  return (
    <div className="welcomeContainer">
      <div className="welcomeWrapper">
        <div className="welcomeContent">
          <h1 className="welcomeHeading">
            Send Crypto
            <br /> across the world
          </h1>
          <p className="welcomeSubheading">
            Explore the world of crypto, and start trading with the best crypto
            exchanges. Buy and sell crypto with the best crypto exchanges.
          </p>
          <button className="welcomeConnetToWalletBtn">
            Connect to Wallet
          </button>
          <div className="welcomeGrid">
            <div className="welcomeGridLeftTop">Reliability</div>
            <div className="welcomeGridLayout">Security</div>
            <div className="welcomeGridRightTop">Etherum</div>
            <div className="welcomeGridLeftBottom">Web3.0</div>
            <div className="welcomeGridLayout">Low Fees</div>
            <div className="welcomeGridRightBottom">Blockchain</div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Welcome;
