

export interface ResourceMeta {
    resourceType: 'table' | 'file'
    name: string
    data: string
}

export enum TaskLogLevel {
    Warn = "Warn",
    Info = "Info",
    Error = "Error"
}

export interface TaskLog {
    timestamp: number;
    message: string;
    level: TaskLogLevel;
}

export interface CodeFile {
    name: string
    path: string
    content: string
    applied?: boolean
}

export interface TaskResult {
    data?: any
    type: string
}