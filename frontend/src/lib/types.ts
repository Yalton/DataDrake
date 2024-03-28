export interface Job {
    id: string;
    root_path: string;
    state: {
        Completed?: string;
        Failed?: string;
        InProgress?: boolean;
        Pending?: boolean;
    };
    created_at: string;
}