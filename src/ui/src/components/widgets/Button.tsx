import './Button.css';

interface Props {
  size?: 'small' | 'medium' | 'large';
  color?: 'green' | 'red' | 'blue';
  disabled?: boolean;
  variant?: string;
  style?: any;
  onClick?: any;
  children: any;
  to?: string;
}

export function Button({
  size = 'small',
  color = null,
  disabled = false,
  onClick = null,
  children = '',
  variant = 'outlined',
  style = null,
  to = null,
}: Props) {
  return (
    <button type="button" className={`${size} ${color || ''} ${variant} button`} style={style || {}} onClick={onClick} disabled={disabled}>
      {children}
    </button>
  );
}
