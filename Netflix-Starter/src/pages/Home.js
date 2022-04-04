import "./Home.css";
import { Logo } from "../images/Netflix";
import { Button, ConnectButton, Icon, Modal, Tab, TabList } from "web3uikit";
import { movies } from "../helpers/library";
import { useState } from "react";
import { Link } from "react-router-dom";
const Home = () => {
  const [visibility, setVisibility] = useState(false);
  const [selectedFilm, setSelectedFilm] = useState(false);

  return (
    <div>
      <div className="logo">
        <Logo />
      </div>
      <div className="connect">
        <Icon fill="#fff" size={24} svg="bell" />
        <ConnectButton />
      </div>

      <div className="topBanner">
        <TabList defaultActiveKey={1} tabStyle="bar">
          <Tab tabKey={1} tabName="Movies">
            <div className="scene">
              <img src={movies[0].Scene} alt="scene" className="sceneImg" />
              <img src={movies[0].Logo} alt="logo" className="sceneLogo" />
              <p className="sceneDesc">{movies[0].Description}</p>
              <div className="playButton">
                <Button
                  theme="Secondary"
                  onClick={() => {
                    window.location.href = "/player";
                  }}
                  type="button"
                  icon="chevronRightX2"
                  text="Play"
                  loadingText="..."
                />
                <Button
                  theme="translucent"
                  type="button"
                  icon="plus"
                  text="Add to Playerlist"
                  loadingText="..."
                />
              </div>
            </div>
            <div className="title">Movies</div>
            <div className="thumbs">
              {movies.map((movie, index) => (
                <>
                  <img
                    src={movie.Thumnbnail}
                    alt="scene"
                    key={index}
                    className="thumbnail"
                    onClick={() => {
                      setSelectedFilm(movie);
                      setVisibility(true);
                    }}
                  />
                </>
              ))}
            </div>
          </Tab>
          <Tab tabKey={2} tabName="Series" isDisabled={true} />
          <Tab tabKey={3} tabName="My List" />
        </TabList>
        {selectedFilm && (
          <div className="modal">
            <Modal
              onCloseButtonPressed={() => setVisibility(false)}
              isVisible={visibility}
              hasFooter={false}
            >
              <div className="modalContent">
                <img
                  src={selectedFilm.Scene}
                  alt="scene"
                  className="modalImg"
                />
                <img src={selectedFilm.Logo} alt="logo" className="modalLogo" />
                <div className="modalPlayButton">
                  <Link to={`/player`}>
                    <Button
                      theme="Secondary"
                      type="button"
                      icon="chevronRightX2"
                      text="Play"
                      loadingText="..."
                    />
                  </Link>
                  <Button
                    theme="translucent"
                    type="button"
                    icon="plus"
                    text="Add to Playerlist"
                    loadingText="..."
                  />
                </div>
                {/* <p className="modalDesc">{selectedFilm.Description}</p> */}
                <div className="movieInfo">
                  <div className="description">
                    <div className="details">
                      <span>{selectedFilm.Year}</span>
                      <span>{selectedFilm.Duration}</span>
                    </div>

                    {selectedFilm.Description}
                  </div>
                  <div className="detailedInfo">
                    Genre : <span className="deets">{selectedFilm.Genre}</span>
                    <br />
                    Actors :{" "}
                    <span className="deets">{selectedFilm.Actors}</span>
                  </div>
                </div>
              </div>
            </Modal>
          </div>
        )}
      </div>
    </div>
  );
};

export default Home;
