ALTER TABLE logs
ADD COLUMN search_vector tsvector;

UPDATE logs
SET search_vector =
to_tsvector(
    'english',
    coalesce(source_name,'') || ' ' ||
    coalesce(event_type,'') || ' ' ||
    coalesce(severity,'') || ' ' ||
    coalesce(message,'')
);

CREATE INDEX logs_search_idx
ON logs
USING GIN(search_vector);