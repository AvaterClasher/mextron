---
template: post
title: blog
author: Soumyadip Moni
author_link: https://github.com/AvaterClasher
date_published: 23 November, 2023
footnote: FYI, you can use Markdown syntax in this page.
---

When I first tried CSS-in-JS libraries like [Styled Components](https://styled-components.com/) and [Emotion](https://emotion.sh), the thing that felt right about it was passing values or state directly into the styles for a component. It really closed the loop with the concept of React where the UI is a function of state. While this was a definite advancement over the traditional way of styling React with classes and pre-processed CSS, it still had its problems.

To highlight some examples, I'll break down some typical examples using two main types of dynamic styles you'll run into with React components:

1. **Values:** like a color, delay, or position. Anything that represents a single value for a CSS property.
1. **States:** like a primary button variant, or a loading state each having their own set of associated styles.

## Where we are today

If you're already familiar with the problem, [skip to the solution](#theres-a-better-way-vanilla-css).

### Values

Using vanilla CSS, or pre-processed CSS by means of LESS or SCSS, the traditional way of passing a _value_ to your styles on was to just use inline styles. So if we have a button component that allows a color, it would look something like this:

```jsx
function Button({ color, children }) {
  return (
    <button className="button" style={{ backgroundColor: color }}>
      {children}
    </button>
  );
}
```

The problem with this approach is that it brings with it all the problems of inline styles. It now has higher specificity making it harder to override, and the styles aren't co-located with the rest of our button styles.

CSS-in-JS (in the case of Styled Components or Emotion) solved this problem by allowing dynamic values like this to be directly as props

```jsx
// We can pass the `color` value into the styled component as a prop
function Button({ color, children }) {
  return <StyledButton color={color}>{children}</StyledButton>;
}

// The syntax is a little funky, but now in the styled component's styles
// we can use its props as a function
const StyledButton = styled.button`
  border: 0;
  border-radius: 4px;
  padding: 8px 12px;
  font-size: 14px;
  color: dimgrey;
  background-color: ${props => props.color};
`;
```

### States

Traditionally, we'd use css classes and concatenate strings. This always felt messy and clunky, but it works nicely on the css side, particularly if you're using a naming convention like BEM along with a pre-processors. Say we have small, medium, and large button sizes, and a primary variant, it might look something like this:

```jsx
function Button({ color, size, primary, children }) {
  return (
    <button
      className={['button', `button--${size}`, primary ? 'button--primary' : null]
        .filter(Boolean)
        .join(' ')}
      style={{ backgroundColor: color }}
    >
      {children}
    </button>
  );
}
```

```scss
.button {
  border: 0;
  border-radius: 4px;
  padding: 8px 12px;
  font-size: 14px;
  color: dimgrey;
  background-color: whitesmoke;

  &--primary {
    background-color: $primary-color;
  }

  &--small {
    height: 30px;
  }

  &--medium {
    height: 40px;
  }

  &--large {
    height: 60px;
  }
}
```

The SCSS is looking nice and clean. I've always liked the pattern of using nesting to concatenate elements and modifiers in SCSS using the BEM syntax.

Our JSX, however, isn't faring so well. That string concatenation on the `className` in the is a mess. The size property isn't too bad, because we're appending the value directly onto the class. The primary variant though... yuck. Not to mention the wacky `filter(Boolean)` in there to prevent a double space in the class list for non-primary buttons. There are better ways of handling this, for example the `classnames` package on NPM. But they only make the problem marginally more bearable.

Unlike dynamic values, Styled Components is still a bit cumbersome in dealing with states

```jsx
function Button({ color, size, primary, children }) {
  return (
    <StyledButton color={color}>{children}</StyledButton>
  )
};

const StyledButton = styled.button`
  border: 0;
  border-radius: 4px;
  padding: 8px 12px;
  font-size: 14px;
  color: dimgrey;
  background-color: whitesmoke;

  ${props => props.primary && css`
    background-color: $primary-color;
  `}

  ${props => props.size === 'small' && css`
    height: 30px;
  `}

  ${props => props.size === 'medium' && css`
    height: 40px;
  `}

  ${props => props.size === 'large' && css`
    height: 60px;
  `}
`;
```

It's not _terrible_, but the repeated functions to grab props gets repetitive and makes reading styles quite noisy. It can also get way worse depending on the type of state. If you have separate but mutually exclusive states sometimes it calls for a ternary expression that can end up looking even more convoluted and difficult to parse.

```jsx
const StyledButton = styled.button`
  border: 0;
  border-radius: 4px;
  padding: 8px 12px;
  font-size: 14px;
  color: dimgrey;

  ${props =>
    props.primary
      ? css`
          height: 60px;
          background-color: darkslateblue;
        `
      : css`
          height: 40px;
          background-color: whitesmoke;
        `}
`;
```

If you're using Prettier for code formatting like I do, you'll end up with a monstrosity like you see above. Monstrosity is a strong way of putting it, but I find the indentation and formatting really difficult to read.

---