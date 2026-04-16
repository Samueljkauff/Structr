
export interface History {
    id: number,
    fileName: string | undefined
    fromPath: string,
    toPath: string,
    movedAt: string | null,
}

export interface RawHistory {
  id: number;
  from_path: string;
  to_path: string;
  moved_at: string | null;
}