import {ChangeEvent, MouseEvent, useState} from 'react';
import {Button} from './Button';
import {SpanBox} from './Box';
import * as icons from './Icons';
import './DataTable.css';

function descendingComparator<T>(a: T, b: T, orderBy: keyof T, defaultVal: any) {
    if ((b[orderBy] || defaultVal) < (a[orderBy] || defaultVal)) {
        return -1;
    }
    if ((b[orderBy] || defaultVal) > (a[orderBy] || defaultVal)) {
        return 1;
    }
    return 0;
}

type Order = 'asc' | 'desc';

function getComparator<T>(order: Order, orderBy: keyof T, numeric: boolean): (a: T, b: T) => number {
    return order === 'desc'
        ? (a: T, b: T) => descendingComparator<T>(a, b, orderBy, numeric ? 0 : '')
        : (a: T, b: T) => -descendingComparator<T>(a, b, orderBy, numeric ? 0 : '');
}

interface DataTableProps<T> {
    rowButtons: any[];
    headCells: HeadCell<T>[];
    onRequestSort: (event: MouseEvent<unknown>, property: keyof T) => void;
    order: Order;
    orderBy: string;
}

export interface HeadCell<T> {
    disablePadding: boolean;
    id: keyof T;
    label: string;
    numeric: boolean;
    formatter?: (row: T, input: any) => any;
}

export interface ButtonInfo<T> {
    label: string;
    callback: (row: T) => Promise<void>;
    activeCallback: (row: T) => boolean;
}

type ButtonInfoFn<T> = (row: T) => ButtonInfo<T>;

export interface Props<T> {
    rows: T[];
    headCells: HeadCell<T>[];
    mainColumn: keyof T;
    initialSortColumn: keyof T;
    initialSortOrder?: Order;
    secondarySortColumn?: keyof T;
    secondarySortOrder?: Order;
    rowButtons: ButtonInfoFn<T>[];
    initialRowsPerPage?: number;
    minWidth?: number | string;
    selectedRows?: any[];
    storageKey?: string;
}

function TableSortLabel({active, direction, onClick, children}: { active: boolean; direction: Order; onClick: any; children: any }) {
    return (
        <SpanBox className="SpanBox" onClick={onClick}>
            {active ? <icons.UpDownArrow flipy={direction === 'asc'}/> : null} {children}
        </SpanBox>
    );
}

function DataTableHead<T>(props: DataTableProps<T>) {
    const {headCells, order, orderBy, onRequestSort, rowButtons = []} = props;
    const createSortHandler = (property: keyof T) => (event: MouseEvent<unknown>) => {
        onRequestSort(event, property);
    };

    return (
        <thead>
        <tr>
            {headCells.map(headCell => (
                <th key={headCell.id.toString()} align={headCell.numeric ? 'right' : 'left'} className="datatable">
                    <TableSortLabel
                        active={orderBy === headCell.id}
                        direction={orderBy === headCell.id ? order : 'asc'}
                        onClick={createSortHandler(headCell.id)}
                    >
                        {headCell.label}
                    </TableSortLabel>
                </th>
            ))}
            {rowButtons && rowButtons.length ? (
                <th className="datatable" key="buttons">
                    &nbsp;
                </th>
            ) : null}
        </tr>
        </thead>
    );
}

export function TablePagination({
                                    rowsPerPageOptions,
                                    count,
                                    rowsPerPage,
                                    page,
                                    onPageChange,
                                    onRowsPerPageChange,
                                }: {
    rowsPerPageOptions: number[];
    count: number;
    rowsPerPage: number;
    page: number;
    onPageChange: any;
    onRowsPerPageChange: any;
}) {
    const maxPage = Math.floor(count / rowsPerPage);

    return (
        <div className="table_pag_main">
            <div
                style={{
                    flex: '1 1 100%',
                    display: 'flex',
                    margin: 0,
                    padding: 0,
                }}
            />
            <span className="pagination" style={{marginLeft: '.5rem', marginRight: '.5rem', display: 'flex', flexShrink: 0}}>
        Rows:
      </span>
            <div className="pagination-select">
                <select className="pagination" name="rows_per_page" onChange={onRowsPerPageChange} value={rowsPerPage}>
                    {rowsPerPageOptions.map(p => (
                        <option className="pagination" key={p} value={p}>
                            {p}
                        </option>
                    ))}
                </select>
            </div>
            <span
                className="pagination"
                style={{
                    display: 'flex',
                    flexShrink: 0,
                }}
            >
        {page * rowsPerPage + 1} - {Math.min(count, (page + 1) * rowsPerPage)} of {count}
      </span>
            <div style={{position: 'relative', display: 'inline-flex', marginLeft: '10px', marginRight: '5px'}}>
                <button className="pagination" type="button" onClick={() => onPageChange(Math.max(0, page - 1))} disabled={page <= 0}>
                    <icons.RightLeftArrow height="100%" width="100%"/>
                </button>
            </div>
            <div style={{position: 'relative', display: 'inline-flex'}}>
                <button className="pagination" type="button" onClick={() => onPageChange(Math.min(maxPage, page + 1))} disabled={page >= maxPage}>
                    <icons.RightLeftArrow height="100%" width="100%" flipx/>
                </button>
            </div>
        </div>
    );
}

export function DataTable<T>({
                                 rows,
                                 headCells,
                                 mainColumn,
                                 initialSortColumn,
                                 initialSortOrder = 'asc',
                                 secondarySortColumn = null,
                                 secondarySortOrder = null,
                                 rowButtons = null,
                                 initialRowsPerPage = 10,
                                 minWidth = 600,
                                 selectedRows = [],
                                 storageKey = null,
                             }: Props<T>) {
    const genStorageKey = (subkey: string) => {
        return `datatable-${storageKey}-${subkey}` || `datatable-${String(mainColumn)}-${String(initialSortColumn)}-${subkey}`;
    };

    const [order, setOrder] = useState<Order>(() => {
        return (localStorage.getItem(genStorageKey('order')) as Order) || initialSortOrder;
    });
    const [orderBy, setOrderBy] = useState<keyof T>(() => {
        const value = localStorage.getItem(genStorageKey('column'));
        return (JSON.parse(value) as keyof T) || initialSortColumn;
    });
    const [page, setPage] = useState(0);
    const [rowsPerPage, setRowsPerPage] = useState(() => {
        return JSON.parse(localStorage.getItem(genStorageKey('rpp'))) || initialRowsPerPage;
    });

    const handleRequestSort = (event: MouseEvent<unknown>, property: keyof T) => {
        const isAsc = orderBy === property && order === 'asc';
        const orderVal = isAsc ? 'desc' : 'asc';
        localStorage.setItem(genStorageKey('column'), JSON.stringify(property));
        localStorage.setItem(genStorageKey('order'), orderVal);
        setOrder(orderVal);
        setOrderBy(property);
    };

    const handleChangePage = (newPage: number) => {
        setPage(newPage);
    };

    const handleChangeRowsPerPage = (event: ChangeEvent<HTMLInputElement>) => {
        const value = parseInt(event.target.value, 10);
        localStorage.setItem(genStorageKey('rpp'), JSON.stringify(value));
        setRowsPerPage(value);
        setPage(0);
    };

    const handleButtonClick = (row: T, fn: (row: T) => Promise<void>) => {
        fn(row);
    };

    const sortCol = headCells.find(h => h.id === orderBy);

    return (
        <div>
            <table style={{width: '100%'}} className="datatable">
                <DataTableHead
                    headCells={headCells}
                    order={order}
                    orderBy={orderBy.toString()}
                    onRequestSort={handleRequestSort}
                    rowButtons={rowButtons}
                />
                <tbody>
                {rows
                    .slice()
                    .sort(getComparator(order, orderBy, sortCol.numeric))
                    .slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage)
                    .map((row, index) => {
                        return (
                            <tr tabIndex={-1} key={row[mainColumn].toString()} className={selectedRows.includes(row[mainColumn]) ? 'selected' : null}>
                                {headCells.map(c => (
                                    <td key={c.id.toString()} align={c.numeric ? 'right' : 'left'} className="datatable">
                                        {c.formatter ? c.formatter(row, row[c.id]) : row[c.id]}
                                    </td>
                                ))}
                                {rowButtons && rowButtons.length ? (
                                    <td key="buttons" className="datatable">
                                        {rowButtons.map(b => {
                                            const buttonInfo = b(row);

                                            return (
                                                <Button
                                                    size="small"
                                                    color="blue"
                                                    key={buttonInfo.label}
                                                    disabled={!buttonInfo.activeCallback(row)}
                                                    onClick={(event: any) => handleButtonClick(row, buttonInfo.callback)}
                                                    variant="contained"
                                                >
                                                    {buttonInfo.label}
                                                </Button>
                                            );
                                        })}
                                    </td>
                                ) : null}
                            </tr>
                        );
                    })}
                </tbody>
            </table>
            <TablePagination
                rowsPerPageOptions={[5, 10, 15, 20, 25, 50]}
                count={rows.length}
                rowsPerPage={rowsPerPage}
                page={page}
                onPageChange={handleChangePage}
                onRowsPerPageChange={handleChangeRowsPerPage}
            />
        </div>
    );
}
