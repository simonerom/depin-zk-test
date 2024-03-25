# Re-importing necessary libraries after code execution state reset
import pandas as pd
import numpy as np

# Generating 100 random records
data_100 = {
    "device_id": np.random.randint(0, 4, size=100),
    "connections": np.random.randint(0, 6, size=100)
}

# Creating a DataFrame for 100 records
df_100 = pd.DataFrame(data_100)

# Checking the DataFrame content for 100 records
df_100_csv_content = df_100.to_csv(index=False)
df_100_csv_content

