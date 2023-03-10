select guid
from tadyen_raw_notification 
where created_date > ? 
order by created_date asc 
limit ?;