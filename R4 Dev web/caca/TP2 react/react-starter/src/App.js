import logo from './logo.svg';
import './App.css';
import Formulaire from './components/Formulaire';

function App() {
  return (
    <div className="m-5">
      <header className="title">
        <h1 className='text-center'>Mon formulaire</h1>
      </header>

      <div className="center">
        {/*rer*/}
        <Formulaire />
      </div>
    </div>
  );
}

export default App;
