import yaml
import os
import sys

def validate_mission_file(file_path):
    try:
        with open(file_path, 'r') as f:
            data = yaml.safe_load(f)
            if not isinstance(data, list):
                # Check if it's a single mission or a batch
                missions = [data] if 'id' in data else []
            else:
                missions = data
            
            for mission in missions:
                required = ['id', 'title', 'description', 'reward', 'objectives']
                for field in required:
                    if field not in mission:
                        print(f"[-] Missing required field '{field}' in mission {mission.get('id', 'unknown')}")
                        return False
                
                for obj in mission['objectives']:
                    if 'id' not in obj or 'type' not in obj:
                        print(f"[-] Invalid objective in mission {mission.get('id')}")
                        return False
        return True
    except Exception as e:
        print(f"[-] Error parsing {file_path}: {e}")
        return False

def main():
    target_dir = sys.argv[1] if len(sys.argv) > 1 else 'missions'
    valid = True
    for root, dirs, files in os.walk(target_dir):
        for file in files:
            if file.endswith('.yaml'):
                full_path = os.path.join(root, file)
                if not validate_mission_file(full_path):
                    valid = False
                else:
                    print(f"[+] {full_path} is valid.")
    
    if not valid:
        sys.exit(1)

if __name__ == "__main__":
    main()
