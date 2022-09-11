export function UpDownArrow({ size = 1.0, flipy = false }: { size?: number; flipy?: boolean }) {
  return (
    <svg
      focusable="false"
      aria-hidden="true"
      viewBox="0 0 24 24"
      width={`${size}em`}
      height={`${size}em`}
      transform={`scale(1,${flipy ? -1 : 1})`}
    >
      <path d="M20 12l-1.41-1.41L13 16.17V4h-2v12.17l-5.58-5.59L4 12l8 8 8-8z" />
    </svg>
  );
}

export function RightLeftArrow({
  size = 1.0,
  flipx = false,
  width = null,
  height = null,
}: {
  size?: number;
  flipx?: boolean;
  width?: string;
  height?: string;
}) {
  return (
    <svg
      focusable="false"
      aria-hidden="true"
      viewBox="0 0 24 24"
      data-testid="KeyboardArrowLeftIcon"
      width={width || `${size}em`}
      height={height || `${size}em`}
      transform={`scale(${flipx ? -1 : 1}, 1)`}
    >
      <path d="M15.41 16.09l-4.58-4.59 4.58-4.59L14 5.5l-6 6 6 6z" />
    </svg>
  );
}
