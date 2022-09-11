import './Select.css';
import { useState } from 'react';

export function Select({
  label,
  value,
  style,
  items,
  onChange = null,
  onKeyDown = null,
  disabled = false,
}: {
  label: string;
  style: any;
  items: any;
  value: string | number;
  onChange?: any;
  onKeyDown?: any;
  disabled?: boolean;
}) {
  const [open, setOpen] = useState(false);
  const [displayState, setDisplayState] = useState(false);
  const doChange = (val: string | number) => {
    setOpen(false);
    onChange(Array.isArray(val) ? val[0] : val);
  };

  // This is kind of gross but the idea is that we need to monitor the state of the details
  // entry, and capture organic interaction with it, so that we know when to close it and when
  // not to.
  const onToggle = (o: boolean, ds: boolean) => {
    if (!o) {
      if (ds) {
        setDisplayState(false);
      } else {
        setDisplayState(true);
        setOpen(true);
      }
    } else if (ds) {
      setDisplayState(false);
      setOpen(false);
    } else {
      setDisplayState(true);
    }
  };

  return (
    <details className="custom-select" open={open} style={style} onToggle={() => onToggle(open, displayState)}>
      <summary className="custom-select">
        {items.map((i: any) => (
          <input
            className="custom-select"
            key={i}
            type="radio"
            checked={(Array.isArray(i) ? i[0] : i) === value}
            title={`[${label}] ${Array.isArray(i) ? i[1] : value}`}
            onChange={() => null}
            onClick={() => setOpen(true)}
          />
        ))}
      </summary>
      <ul className="custom-select">
        {items.map((i: any) => {
          const labelStyle = (Array.isArray(i) ? i[0] : i) === value ? { color: 'red' } : null;
          return (
            <li className="custom-select" key={i}>
              <label className="custom-select" onClick={() => doChange(i)} htmlFor={Array.isArray(i) ? i[0] : i} style={labelStyle}>
                {Array.isArray(i) ? i[1] : i}
              </label>
            </li>
          );
        })}
      </ul>
    </details>
  );
}
