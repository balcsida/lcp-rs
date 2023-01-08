// Find the services to be deployed in a directory
fn find_services(dir: &str) -> Vec<Service> {
    let mut services = Vec::new();

    // Read the contents of the directory
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error reading directory {}: {}", dir, err);
            return services;
        }
    };

    // Iterate over the entries
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error reading directory entry: {}", err);
                continue;
            }
        };

        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(err) => {
                eprintln!("Error getting file type: {}", err);
                continue;
            }
        };

        // Check if the entry is a directory
        if file_type.is_dir() {
            // Recursively search for services in the directory
            services.extend(find_services(path.to_str().unwrap()));
        } else if file_type.is_file() {
            // Check if the entry is a service file
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name == "service.json" {
                // Parse the service file
                let service = match parse_service
