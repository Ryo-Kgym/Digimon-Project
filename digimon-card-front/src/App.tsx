import React from 'react';
import './App.css';
import AttackArea from "./component/fight/AttackField";

function App() {

    return (
        <div className="App">
            <AttackArea
                primaryAttack={{value: 300}}
                secondaryAttack={{value: 200}}
                tertiaryAttack={{value: 100}}
            />
        </div>
    );
}

export default App;
