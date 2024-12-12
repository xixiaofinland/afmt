class SOSLClass {{
    searchResult = [ FIND 'San Francisco' RETURNING My_Custom_Object__c( Id WHERE DISTANCE(My_Location_Field__c, GEOLOCATION(37, 122), 'mi') < 100) ];
    }}
