// @ts-check

import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.strictTypeChecked,
  // ...tseslint.configs.stylisticTypeChecked,
  {
    languageOptions: {
      parserOptions: {
        project: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
  {
    "rules": {
      "@typescript-eslint/no-unused-vars": ["off"],
      "@typescript-eslint/restrict-template-expressions": ["error",
        {
          allowAny: true,
          allowBoolean: true,
          allowNullish: true,
          allowNumber: true,
          allowRegExp: true,
        }],
    },
  },
);


//     "@typescript-eslint/no-unused-vars": [
//       "error",
//       {
//         "args": "all",
//         "argsIgnorePattern": "^_",
//         "caughtErrors": "all",
//         "caughtErrorsIgnorePattern": "^_",
//         "destructuredArrayIgnorePattern": "^_",
//         "varsIgnorePattern": "^_",
//         "ignoreRestSiblings": true
//       }
//     ]
//   }
