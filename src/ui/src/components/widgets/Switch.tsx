import './Switch.css';

export function Switch({ label = '', checked, onChange }: { label: string; checked: boolean; onChange: (val: boolean) => void }) {
  return (
    <span className="custom-switch">
      <input type="checkbox" checked={checked} onChange={() => onChange(!checked)} />
      <span>{label}</span>
    </span>
  );
}
