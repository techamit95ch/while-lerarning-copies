import {
  NavBar,
  Welcome,
  Header,
  Footer,
  Services,
  Transaction,
} from "./components";

const App = () => {
  return (
    <div className="min-h-screen">
      <Header>
        <NavBar />
        <Welcome />
      </Header>
      <Transaction />
      <Services />
      <Footer />
    </div>
  );
};

export default App;
