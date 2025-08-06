
CREATE TABLE IF NOT EXISTS jobs (
    id TEXT,
    title TEXT,
    description TEXT
);

CREATE INDEX index_jobs ON jobs(id);

CREATE TABLE IF NOT EXISTS assignee (
    id TEXT,
    job_id TEXT,
    name TEXT,
    email TEXT
);


CREATE TABLE IF NOT EXISTS fine (
    id TEXT,
    category TEXT,
    title TEXT,
    description TEXT,
    fine REAL
);
