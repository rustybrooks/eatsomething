import './TextInput.css';
import { ForwardedRef, forwardRef } from 'react';

interface Props {
  label: string;
  style?: any;
  value?: string | number;
  onChange?: any;
  onKeyPress?: any;
  disabled?: boolean;
  error?: boolean;
  helperText?: string;
  autoFocus?: boolean;
  type?: string;
  ref?: any;
}

export const TextInput = forwardRef((props: Props, ref: ForwardedRef<HTMLInputElement>) => {
  return (
    <div className="text-input" style={props.style}>
      <input
        className="text-input"
        autoFocus={props.autoFocus}
        type={props.type}
        disabled={props.disabled}
        value={props.value}
        placeholder={props.label}
        onChange={props.onChange}
        onKeyPress={props.onKeyPress}
        ref={ref}
      />
      {props.error ? <p className="text-input-error">{props.helperText}</p> : null}
    </div>
  );
});
