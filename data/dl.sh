#! /bin/bash

curl https://raw.githubusercontent.com/xivapi/ffxiv-datamining/master/csv/Recipe.csv > Recipe.csv
sed -i '1d;3d' Recipe.csv

curl https://raw.githubusercontent.com/xivapi/ffxiv-datamining/master/csv/RecipeLevelTable.csv > RecipeLevelTable.csv
sed -i '1d;3d' RecipeLevelTable.csv

curl https://raw.githubusercontent.com/xivapi/ffxiv-datamining/master/csv/Item.csv > Item.csv
sed -i '1d;3d' Item.csv

curl https://raw.githubusercontent.com/xivapi/ffxiv-datamining/master/csv/RecipeLookup.csv > RecipeLookup.csv
sed -i '1d;3d' RecipeLookup.csv

curl https://raw.githubusercontent.com/xivapi/ffxiv-datamining/master/csv/Action.csv > Action.csv
sed -i '1d;3d' Action.csv

curl https://raw.githubusercontent.com/xivapi/ffxiv-datamining/master/csv/CraftAction.csv > CraftAction.csv
sed -i '1d;3d' CraftAction.csv

curl https://raw.githubusercontent.com/xivapi/ffxiv-datamining/master/csv/Status.csv > Status.csv
sed -i '1d;3d' Status.csv