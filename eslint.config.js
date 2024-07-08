// @ts-check

import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.recommendedTypeChecked,
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
      "@typescript-eslint/no-unsafe-assignment": ["off"],
      "@typescript-eslint/no-unsafe-argument": ["off"],
      "@typescript-eslint/no-unsafe-member-access": ["off"],
      "@typescript-eslint/no-unsafe-call": ["off"],
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
