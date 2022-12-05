
# CSV standard
Weapons are defined as single csv files, delimited by `,` and terminated by `;`.  
Every text/string column must be enclosed by a pair of `"`, delimiters or terminators within the text/string are forbidden.  
Whitespace in the weapon name is replaced by following characters to make them file-system compatible, with following replacements:  
 * ` ` (Spacebar) => `_` (underscore)

Whitespaces such as  `\n` or ` ` (space) have no effect on the CSV, unlike other common standards. 

## Rows
The first row is a header defining the weapon-class and other WIP headers

Any successive rows are indexed as following `Weapon level to unlock row` `Name of Item` `(Optional) Unlock-Type`  
 
Unlock-Types are as following:
 * None specified means regular attachment
 * Other Weapon unlock
 * Attachment category