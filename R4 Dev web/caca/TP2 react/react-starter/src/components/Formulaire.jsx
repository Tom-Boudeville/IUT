import React from 'react'
import FormBlockButton from './FormBlockButton'
import FormBlockInput from './FormBlockInput'

const Formulaire = () => {
  return (
    <>
        <div>Formulaire</div>
        
        <div>
          <FormBlockInput id="txtName" name="name" label="Nom :"/>
        </div>

        <div>
          <FormBlockInput id="txtFirst-name" name="first-name" label="Préom :"/>
        </div>
        
        <div>
          <FormBlockInput id="txtMail" name="mail" label="Mail :"/>
        </div>
        
        <FormBlockButton name="Valider"/>
    </>

  )
}

export default Formulaire