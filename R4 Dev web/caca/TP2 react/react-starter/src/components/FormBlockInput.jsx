const FormBlockInput= (props) => (
    <div className="mb-3">
    <label htmlFor={props.id}>{props.label}</label>
    <input id={props.id} name={props.name} className="form-control" />
    </div>);
   export default FormBlockInput;
   